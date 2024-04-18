use std::path::Path;

use build_utils::{artifact::Artifact, WPI_VERSION};

use crate::WPI_MAVEN as MAVEN;

pub fn get_artifacts() -> Vec<Artifact> {
    vec![
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("runtime".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("embcanshim".to_owned())
            .no_deploy()
            .no_headers()
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("runtime".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("fpgalvshim".to_owned())
            .no_deploy()
            .no_headers()
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("chipobject".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("RoboRIO_FRC_ChipObject".to_owned())
            .no_deploy()
            .no_headers()
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("netcomm".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("FRC_NetworkCommunication".to_owned())
            .no_deploy()
            .no_headers()
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("visa".to_owned())
            .version("2024.2.1".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("visa".to_owned())
            .no_deploy()
            .no_headers()
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpiHal".to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpiutil".to_owned())
            .build()
            .unwrap(),
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpimath".to_owned())
            .build()
            .unwrap(),
    ]
}

pub fn get_allow_list() -> &'static str {
    "(HAL|WPI)_.*"
}

pub fn get_start_path() -> [&'static Path; 1] {
    [Path::new("hal/HAL.h")]
}
