use std::path::Path;

use build_utils::artifact::Artifact;

use crate::{REV_MAVEN, WPI_MAVEN};

pub fn get_artifacts() -> Vec<Artifact> {
    vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("com.revrobotics.frc".to_owned())
            .artifact_id("REVLib-driver".to_owned())
            .version("2024.2.4".to_owned())
            .maven_url(REV_MAVEN.to_owned())
            .lib_name("REVLibDriver".to_owned())
            .build()
            .unwrap(),
    ]
}

pub fn get_allow_list() -> &'static str {
    "c_(SparkMax|REVLib)_.*"
}

pub fn get_start_path() -> &'static Path {
    Path::new("rev/CANSparkMaxDriver.h")
}
