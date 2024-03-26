use std::path::{Path, PathBuf};

use cargo_deploy::{
    create_target, Action, DeployCode, DeployLibraries, DeployStartCommand, ProgramKill,
    StartProgram, TeamNumber,
};
use clap::Parser;
use tokio::process::Command;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Args {
    Deploy(DeployArgs),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct DeployArgs {
    #[arg(short, long)]
    executable: Option<PathBuf>,
    #[arg(short, long)]
    libs: Option<PathBuf>,
    #[arg(short, long)]
    release: Option<bool>,
    #[arg(short, long)]
    build: Option<bool>,
}

#[tokio::main]
async fn main() {
    let Args::Deploy(args) = Args::parse();

    let release = args.release.unwrap_or(false);

    if !matches!(args.build, Some(false)) {
        let mut cmd = Command::new("cargo");

        cmd.arg("build");

        if release {
            cmd.arg("--release");
        }

        let status = cmd.status().await.expect("Could not build project");

        if !status.success() {
            return;
        }
    }

    let executable = args.executable.unwrap_or_else(|| {
        let cmd = cargo_metadata::MetadataCommand::new().exec().unwrap();

        let mut path = PathBuf::from(format!(
            "target/arm-unknown-linux-gnueabi/{}/",
            if release { "release" } else { "debug" },
        ));

        let name = cmd
            .workspace_default_packages()
            .into_iter()
            .find_map(|package| {
                package
                    .targets
                    .iter()
                    .find(|target| target.kind.contains(&"bin".to_string()))
                    .map(|target| target.name.clone())
            });

        path.push(name.expect("Could not find binary package"));

        println!("Using executable at path: {}", path.to_str().unwrap());

        path
    });

    let target = create_target(TeamNumber::new(9033))
        .await
        .expect("Could not find target");

    let lib_folder = args.libs.unwrap_or(Path::new("target/lib").to_path_buf());

    ProgramKill.execute(&target).await;

    println!("Deployed kill script");

    DeployStartCommand.execute(&target).await;

    println!("Deployed start command");

    DeployCode {
        local: executable.as_path(),
    }
    .execute(&target)
    .await;

    println!("Deployed user code");

    DeployLibraries {
        libs: lib_folder.as_path(),
    }
    .execute(&target)
    .await;

    println!("Deployed libraries");

    StartProgram.execute(&target).await;
}
