use std::path::Path;

use build_utils::{
    artifact::Artifact,
    build,
};

const WPI_MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";
const NAVX_MAVEN: &str = "https://dev.studica.com/maven/release/2023/";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let headers = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpilibc".to_owned())
            .artifact_id("wpilibc-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .lib_name("wpilibc".to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ntcore".to_owned())
            .artifact_id("ntcore-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("com.kauailabs.navx.frc".to_owned())
            .artifact_id("navx-frc-cpp".to_owned())
            .version("2023.0.3".to_owned())
            .maven_url(NAVX_MAVEN.to_owned())
            .lib_name("NavX".to_owned())
            .build()?,
    ];

    build(&headers, "AHRS_.", &Path::new("AHRS.h")).await?;

    Ok(())
}
