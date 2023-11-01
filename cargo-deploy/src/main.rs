use std::path::PathBuf;

use cargo_deploy::{create_target, TeamNumber, ProgramKill, ProgramRun, ProgramStart, DeployLibs, ConfigureLibs};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    executable: PathBuf,
    #[arg(short, long)]
    libs: PathBuf
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut target = create_target(TeamNumber::new(9033)).await.expect("Could not find target");

    target.run(&mut ProgramKill).await;
    target.run(&mut ProgramRun {
        local: &args.executable
    }).await;
    target.run(&mut DeployLibs {
        dir: &args.libs
    }).await;
    target.run(&mut ConfigureLibs).await;
    target.run(&mut ProgramStart).await;
}
