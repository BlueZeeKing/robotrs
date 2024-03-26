use std::{env, path::Path};

use anyhow::Result;
use build_utils::{artifact::Artifact, build, WPI_VERSION};
use tokio::{fs::File, io::AsyncWriteExt};

const MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";

#[tokio::main]
async fn main() -> Result<()> {
    let libs = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("runtime".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("embcanshim".to_owned())
            .no_deploy()
            .no_headers()
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("runtime".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("fpgalvshim".to_owned())
            .no_deploy()
            .no_headers()
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("chipobject".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("RoboRIO_FRC_ChipObject".to_owned())
            .no_headers()
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("netcomm".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("FRC_NetworkCommunication".to_owned())
            .no_headers()
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("visa".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("visa".to_owned())
            .no_headers()
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpiHal".to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpiutil".to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpimath".to_owned())
            .build()?,
    ];

    if let Some(out_str) = env::var_os("OUT_DIR") {
        let out_dir = Path::new(&out_str);

        let mut version = File::create(out_dir.join("version.txt")).await?;

        version.write_all(WPI_VERSION.as_bytes()).await?;
    }

    build(&libs, "(HAL|WPI)_.*", Path::new("hal/HAL.h")).await
}
