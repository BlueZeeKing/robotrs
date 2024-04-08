use std::{env, path::Path};

use anyhow::Result;
use build_utils::{build, WPI_VERSION};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    let libs = bindings::hal::get_artifacts();

    if let Some(out_str) = env::var_os("OUT_DIR") {
        let out_dir = Path::new(&out_str);

        let mut version = File::create(out_dir.join("version.txt")).await?;

        version.write_all(WPI_VERSION.as_bytes()).await?;
    }

    build(&libs).await
}
