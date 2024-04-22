use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Attribute, FnArg, Ident, ItemFn, Meta, Token, Type};

#[derive(FromMeta)]
struct Args {
    #[darling(default)]
    priority_name: Option<Ident>,
    #[darling(default)]
    function_name: Option<Ident>,
    #[darling(default)]
    wait: Option<bool>,
}

fn attr_matches(attr: &Attribute) -> bool {
    matches!(&attr.meta, Meta::Path(path) if path.is_ident(&Ident::new("subsystem", Span::call_site())))
}

#[proc_macro_attribute]
pub fn subsystem_task(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let parsed_input = parse_macro_input!(input as ItemFn);
    let mut original = parsed_input.clone();

    original.sig.inputs.iter_mut().for_each(|val| match val {
        FnArg::Receiver(val) => val.attrs.retain(|attr| !attr_matches(attr)),
        FnArg::Typed(val) => val.attrs.retain(|attr| !attr_matches(attr)),
    });

    let mut new_fn = parsed_input.clone();

    new_fn.sig.ident = args.function_name.unwrap_or_else(|| {
        Ident::new(
            &format!("{}_subsystem", parsed_input.sig.ident),
            Span::call_site(),
        )
    });
    new_fn.sig.asyncness = Some(Token![async](Span::call_site()));

    let mut subsystems = Vec::new();

    new_fn.sig.inputs.iter_mut().for_each(|arg| {
        if let FnArg::Typed(arg) = arg {
            if !arg.attrs.iter().any(|attr| attr_matches(attr)) {
                return;
            }

            arg.attrs.retain(|attr| !attr_matches(attr));

            subsystems.push(*arg.pat.clone());

            let old_ty = arg.ty.clone();

            let Type::Reference(old_ty) = *old_ty else {
                return;
            };

            if old_ty.mutability.is_none() {
                return;
            }

            let old_ty = old_ty.elem;

            arg.ty = Box::new(parse_quote! {
                &::utils::subsystem::Subsystem<#old_ty>
            });

            return;
        }

        if let FnArg::Receiver(arg) = arg {
            if !arg.attrs.iter().any(|attr| attr_matches(attr)) {
                return;
            }
        }

        subsystems.push(parse_quote! { subsystem });

        *arg = parse_quote! {
            subsystem: &::utils::subsystem::Subsystem<Self>
        };
    });

    let priority_name = args
        .priority_name
        .unwrap_or_else(|| Ident::new("priority", Span::call_site()));

    new_fn.sig.inputs.push(parse_quote! {
        #priority_name: impl ::utils::subsystem::AsPriority + Clone
    });

    let name = parsed_input.sig.ident;

    let func_args = parsed_input.sig.inputs.into_iter().map(|arg| match arg {
        FnArg::Receiver(val) => {
            if val.attrs.iter().any(attr_matches) {
                parse_quote!(&mut subsystem)
            } else {
                parse_quote!(self)
            }
        }
        FnArg::Typed(val) => {
            if val.attrs.iter().any(attr_matches) {
                let val = val.pat;
                parse_quote!(&mut #val)
            } else {
                *val.pat
            }
        }
    });

    let run = if parsed_input.sig.asyncness.is_none() {
        quote! {
            Self::#name(#(#func_args),*)
        }
    } else {
        quote! {
            Self::#name(#(#func_args),*).await
        }
    };

    let wait = if args.wait.unwrap_or(false) {
        quote! {
            let res = #run;

            ::futures::future::pending::<()>().await;

            #(drop(#subsystems);)*

            res
        }
    } else {
        run
    };

    new_fn.block = parse_quote! {
        {
            let (#(mut #subsystems,)*) = futures::join!(#(#subsystems.lock(#priority_name.clone())),*);

            #wait
        }
    };

    quote! {
        #original

        #new_fn

    }
    .into()
}
