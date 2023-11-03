use std::path::Path;

use anyhow::Result;
use build_utils::{
    artifact::Artifact,
    build,
};

const MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";

#[tokio::main]
async fn main() -> Result<()> {
    let headers = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ntcore".to_owned())
            .artifact_id("ntcore-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("ntcore".to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpinet".to_owned())
            .artifact_id("wpinet-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpinet".to_owned())
            .build()?,
    ];

    build(&headers, "NT_.*", Path::new("ntcore.h")).await
}
