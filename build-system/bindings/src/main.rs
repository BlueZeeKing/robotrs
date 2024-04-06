use std::{path::PathBuf, process::exit};

use bindings::{ctre, hal, nt, rev};
use build_utils::gen_bindings;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    #[arg(value_enum)]
    library: Library,
    out: PathBuf,
}

#[derive(Clone, ValueEnum)]
enum Library {
    Rev,
    Ctre,
    Hal,
    Nt,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let result = match args.library {
        Library::Rev => {
            gen_bindings(
                &rev::get_artifacts(),
                rev::get_allow_list(),
                rev::get_start_path(),
                &args.out,
            )
            .await
        }
        Library::Ctre => {
            gen_bindings(
                &ctre::get_artifacts(),
                ctre::get_allow_list(),
                ctre::get_start_path(),
                &args.out,
            )
            .await
        }
        Library::Hal => {
            gen_bindings(
                &hal::get_artifacts(),
                hal::get_allow_list(),
                hal::get_start_path(),
                &args.out,
            )
            .await
        }
        Library::Nt => {
            gen_bindings(
                &nt::get_artifacts(),
                nt::get_allow_list(),
                nt::get_start_path(),
                &args.out,
            )
            .await
        }
    };

    if let Err(err) = result {
        eprintln!("Error occurred: {}", err);
        exit(1);
    }
}
