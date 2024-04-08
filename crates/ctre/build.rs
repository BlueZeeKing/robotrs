use anyhow::Result;
use build_utils::build;

#[tokio::main]
async fn main() -> Result<()> {
    let headers = bindings::ctre::get_artifacts();

    build(&headers).await
}
