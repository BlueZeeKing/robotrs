use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use build_utils::{
    artifact::{Artifact, Target},
    zip::{extract_libs, get_zip, write_archive_to_path},
};
use tempfile::TempDir;

const MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";

#[tokio::main]
async fn main() -> Result<()> {
    let headers = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
    ];

    let libs = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("runtime".to_owned())
            .version("2023.3.0".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("embcanshim".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("runtime".to_owned())
            .version("2023.3.0".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("fpgalvshim".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("chipobject".to_owned())
            .version("2023.3.0".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("RoboRIO_FRC_ChipObject".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("netcomm".to_owned())
            .version("2023.3.0".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("FRC_NetworkCommunication".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ni-libraries".to_owned())
            .artifact_id("visa".to_owned())
            .version("2023.3.0".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("visa".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpiHal".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpiutil".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpimath".to_owned())
            .target(Target::RoboRio)
            .build()?,
    ];

    let tempdir = TempDir::new()?;
    let include_path = tempdir.path().join("include");

    fs::create_dir_all(&include_path)?;

    for header in headers {
        write_archive_to_path(&include_path, get_zip(&header.get_url()).await?)?;
    }

    if let Ok(host) = env::var("HOST") {
        env::set_var("TARGET", host);
    }

    let result = bindgen::Builder::default()
        .clang_args([
            "-xc++",
            "-std=c++20",
            &format!("--include-directory={}", include_path.to_str().unwrap()),
        ])
        .header(include_path.join("hal/HAL.h").to_str().unwrap())
        .blocklist_type("std::.*")
        .blocklist_function("std::.*")
        .blocklist_item("std::.*")
        .allowlist_type("HAL_.*")
        .allowlist_function("HAL_.*")
        .allowlist_var("HAL_.*")
        .generate()?;

    if let Some(out_str) = env::var_os("OUT_DIR") {
        let out_dir = Path::new(&out_str);

        result.write_to_file(out_dir.join("bindings.rs"))?;
    }

    let Some(libs_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
        bail!("Unable to find out dir");
    };

    fs::create_dir_all(&libs_dir)?;

    println!(
        "cargo:rustc-link-search=native={}",
        libs_dir.to_str().unwrap()
    );

    for lib in libs {
        let mut archive = get_zip(&lib.get_url()).await?;
        let inner_libs = extract_libs(&mut archive)?;

        for (name, index) in inner_libs {
            if name == lib.get_lib_name().expect("Could not get lib name") {
                let mut zip_file = archive.by_index(index)?;

                let mut fs_file = File::create(libs_dir.join(format!("lib{name}.so")))?;

                std::io::copy(&mut zip_file, &mut fs_file)?;

                break;
            }
        }

        println!("cargo:rustc-link-lib=dylib={}", lib.get_lib_name().unwrap());
    }

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
