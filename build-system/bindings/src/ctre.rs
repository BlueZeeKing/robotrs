use std::path::Path;

use build_utils::artifact::Artifact;

use crate::{CTRE_MAVEN, WPI_MAVEN};

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
            .group_id("com.ctre.phoenix".to_owned())
            .artifact_id("cci".to_owned())
            .version("5.33.1".to_owned())
            .maven_url(CTRE_MAVEN.to_owned())
            .lib_name("CTRE_PhoenixCCI".to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("com.ctre.phoenix6".to_owned())
            .artifact_id("tools".to_owned())
            .version("24.2.0".to_owned())
            .maven_url(CTRE_MAVEN.to_owned())
            .lib_name("CTRE_PhoenixTools".to_owned())
            .build()
            .unwrap(),
    ]
}

pub fn get_allow_list() -> &'static str {
    "c_MotController_.*"
}

pub fn get_start_path() -> &'static Path {
    Path::new("ctre/phoenix/cci/MotController_CCI.h")
}
