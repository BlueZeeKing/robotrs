use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use build_utils::{
    artifact::{Artifact, Target},
    zip::{get_zip, write_archive_to_path},
};
use tempfile::TempDir;

const MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";

#[tokio::main]
async fn main() -> Result<()> {
    let headers = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.ntcore".to_owned())
            .artifact_id("ntcore-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
    ];

    let libs = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.ntcore".to_owned())
            .artifact_id("ntcore-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("ntcore".to_owned())
            .target(Target::RoboRio)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpinet".to_owned())
            .artifact_id("wpinet-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(MAVEN.to_owned())
            .lib_name("wpinet".to_owned())
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
        .clang_args([&format!(
            "--include-directory={}",
            include_path.to_str().unwrap()
        )])
        .header(include_path.join("ntcore.h").to_str().unwrap())
        .allowlist_type("NT_.*")
        .allowlist_function("NT_.*")
        .allowlist_var("NT_.*")
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

        let mut zip_file = lib.find_lib_in_zip(&mut archive)?;

        let mut fs_file =
            File::create(libs_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

        std::io::copy(&mut zip_file, &mut fs_file)?;

        println!("cargo:rustc-link-lib=dylib={}", lib.get_lib_name().unwrap());
    }

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
