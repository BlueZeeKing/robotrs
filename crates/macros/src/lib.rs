use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, Ident, ItemFn, Token};

#[derive(FromMeta)]
struct Args {
    #[darling(default)]
    priority_name: Option<Ident>,
    #[darling(default)]
    function_name: Option<Ident>,
    #[darling(default)]
    subsystem_name: Option<Ident>,
    #[darling(default)]
    wait: Option<bool>,
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

    let input2: proc_macro2::TokenStream = input.clone().into();
    let parsed_input = parse_macro_input!(input as ItemFn);

    let mut new_fn = parsed_input.clone();

    new_fn.sig.ident = args.function_name.unwrap_or_else(|| {
        Ident::new(
            &format!("{}_subsystem", parsed_input.sig.ident),
            Span::call_site(),
        )
    });
    new_fn.sig.asyncness = Some(Token![async](Span::call_site()));

    let subsystem_name = args
        .subsystem_name
        .unwrap_or_else(|| Ident::new("subsystem", Span::call_site()));

    let first = new_fn
        .sig
        .inputs
        .first_mut()
        .expect("The function must have an argument");

    *first = parse_quote!(#subsystem_name: &::utils::subsystem::Subsystem<Self>);

    let priority_name = args
        .priority_name
        .unwrap_or_else(|| Ident::new("priority", Span::call_site()));

    new_fn.sig.inputs.push(parse_quote! {
        #priority_name: u32
    });

    let name = parsed_input.sig.ident;

    let func_args = parsed_input
        .sig
        .inputs
        .into_iter()
        .skip(1)
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(val) => Some(val),
        })
        .map(|val| val.pat);

    let run = if parsed_input.sig.asyncness.is_none() {
        quote! {
            #subsystem_name.#name(#(#func_args),*)
        }
    } else {
        quote! {
            #subsystem_name.#name(#(#func_args),*).await
        }
    };

    let wait = if args.wait.unwrap_or(false) {
        quote! {
            let res = #run;

            ::futures::future::pending::<()>().await;

            drop(#subsystem_name);

            res
        }
    } else {
        run
    };

    new_fn.block = parse_quote! {
        {
            let mut #subsystem_name = #subsystem_name.lock(#priority_name).await;

            #wait
        }
    };

    quote! {
        #input2

        #new_fn

    }
    .into()
}
