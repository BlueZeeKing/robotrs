#![feature(proc_macro_tracked_env, track_path)]

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    time::Duration,
};

use proc_macro::{tracked_path, TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;
use serde::Deserialize;
use serde_json::from_str;
use syn::{parse, LitStr};

#[derive(Debug, Deserialize)]
struct Path {
    samples: Vec<TrajectoryPoint>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrajectoryPoint {
    x: f32,
    y: f32,
    heading: f32,
    angular_velocity: f32,
    velocity_x: f32,
    velocity_y: f32,
    timestamp: f32,
}

#[derive(Debug)]
struct PartialPathEntry {
    root: Option<Path>,
    values: Vec<(u32, Path)>,
}

fn to_camel_case(val: &str) -> String {
    let mut out = String::with_capacity(val.len());
    let mut should_break = false;

    for letter in val.chars() {
        if letter.is_whitespace() || letter == '_' {
            if should_break {
                out.push('_');
            }
            should_break = false;
        } else if letter.is_uppercase() {
            if should_break {
                out.push('_');
            }
            out.push(letter.to_ascii_lowercase());
            should_break = true;
        } else {
            out.push(letter);
            should_break = true;
        }
    }

    out
}

/// This macros searches a directory (`deploy/choreo` by default) for `.traj` files and serializes
/// them into `Path`s. The macros creates a public `paths` module. Each submodule within that
/// corresponds to one path. There is a `const` called `PATH` within each submodule correspoinding
/// to the entire path. There are is also `PATH1`, `PATH2`, and so one, which correspond with the
/// different segments of the path. The path names are converted to camel case before the modules
/// are created.
///
/// # Example
///
/// ```rust
/// choreo!(); // Load from default directory
/// ```
///
/// ```rust
/// choreo!("/some/arbitrary/directory"); // Load from arbitrary directory
/// ```
#[proc_macro]
pub fn choreo(item: TokenStream) -> TokenStream {
    let path = parse::<LitStr>(item)
        .ok()
        .map(|val| PathBuf::from(val.value()))
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("deploy/choreo")
        });

    tracked_path::path(path.to_str().unwrap());

    let paths = fs::read_dir(path).expect("Could not read trajectory directory");

    let mut trajectories = HashMap::new();

    for file in paths {
        let file = file.expect("Failed to read directory entry");

        let name = file.file_name();
        let name = name.to_str().unwrap().trim_end_matches(".traj");
        let mut parts = name.split('.');

        let name = to_camel_case(parts.next().unwrap());
        let num = parts.next().map(|val| val.parse::<u32>().unwrap());

        if !trajectories.contains_key(&name) {
            trajectories.insert(
                name.to_owned(),
                PartialPathEntry {
                    root: None,
                    values: Vec::new(),
                },
            );
        }

        let mut file = File::open(file.path()).expect("Could not open trajectory file");

        let mut val = String::new();
        file.read_to_string(&mut val)
            .expect("Could not read trajectory file");

        let path: Path = from_str(&val).expect("Could not parse trajectory file");

        if let Some(num) = num {
            trajectories
                .get_mut(&name)
                .unwrap()
                .values
                .push((num, path));
        } else {
            trajectories.get_mut(&name).unwrap().root = Some(path);
        }
    }

    let paths = trajectories.into_iter().map(|(name, entry)| {
        let name = Ident::new(&name, Span::call_site());

        let root_samples = entry
            .root
            .unwrap()
            .samples
            .into_iter()
            .map(sample_to_tokens);

        let other_paths = entry.values.into_iter().map(|(num, path)| {
            let name = Ident::new(&format!("PATH{}", num), Span::call_site());

            let samples = path.samples.into_iter().map(sample_to_tokens);

            quote! {
                pub const #name: ::choreolib::Path<'static> = ::choreolib::Path {
                    samples: &[#(#samples),*],
                };
            }
        });

        quote! {
            pub mod #name {
                pub const PATH: ::choreolib::Path<'static> = ::choreolib::Path {
                    samples: &[#(#root_samples),*],
                };

                #(#other_paths)*
            }
        }
    });

    quote! {
        #[allow(warnings, clippy::approx_constant)]
        pub mod paths {
            #(#paths)*
        }
    }
    .into()
}

fn sample_to_tokens(sample: TrajectoryPoint) -> proc_macro2::TokenStream {
    let x = sample.x;
    let y = sample.y;
    let heading = sample.heading;
    let angular_velocity = sample.angular_velocity;
    let velocity_x = sample.velocity_x;
    let velocity_y = sample.velocity_y;
    let time = Duration::from_secs_f32(sample.timestamp);
    let secs = time.as_secs();
    let nanos = time.subsec_nanos();

    quote! {
        ::choreolib::TrajectoryPoint {
            x: #x,
            y: #y,
            heading: #heading,
            angular_velocity: #angular_velocity,
            velocity_x: #velocity_x,
            velocity_y: #velocity_y,
            timestamp: ::std::time::Duration::new(#secs, #nanos),

        }
    }
}
