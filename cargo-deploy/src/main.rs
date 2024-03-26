use std::path::{Path, PathBuf};

use cargo_deploy::{
    create_target, Action, DeployCode, DeployStartCommand, ProgramKill, StartProgram, TeamNumber,
};
use clap::Parser;

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
    executable: PathBuf,
    #[arg(short, long)]
    libs: PathBuf,
}

#[tokio::main]
async fn main() {
    let Args::Deploy(args) = Args::parse();

    let target = create_target(TeamNumber::new(9033))
        .await
        .expect("Could not find target");

    ProgramKill.execute(&target).await;
    DeployStartCommand.execute(&target).await;
    DeployCode {
        local: args.executable.as_path(),
    }
    .execute(&target)
    .await;
    StartProgram.execute(&target).await;
}
