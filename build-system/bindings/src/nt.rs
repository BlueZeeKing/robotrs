use std::path::Path;

use build_utils::artifact::Artifact;

use crate::WPI_MAVEN as MAVEN;

pub fn get_artifacts() -> Vec<Artifact> {
    vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.ntcore".to_owned())
            .artifact_id("ntcore-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("ntcore".to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpinet".to_owned())
            .artifact_id("wpinet-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpinet".to_owned())
            .build()
            .unwrap(),
    ]
}

pub fn get_allow_list() -> &'static str {
    "NT_.*"
}

pub fn get_start_path() -> [&'static Path; 1] {
    [Path::new("ntcore.h")]
}
