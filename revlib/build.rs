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

const WPI_MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";
const REV_MAVEN: &str = "https://maven.revrobotics.com/";

#[tokio::main]
async fn main() -> Result<()> {
    let headers = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version("2023.4.3".to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
        Artifact::builder()
            .group_id("com.revrobotics.frc".to_owned())
            .artifact_id("REVLib-driver".to_owned())
            .version("2023.1.3".to_owned())
            .maven_url(REV_MAVEN.to_owned())
            .target(Target::Headers)
            .build()?,
    ];

    let libs = vec![Artifact::builder()
        .group_id("com.revrobotics.frc".to_owned())
        .artifact_id("REVLib-driver".to_owned())
        .version("2023.1.3".to_owned())
        .maven_url(REV_MAVEN.to_owned())
        .target(Target::RoboRio)
        .lib_name("REVLibDriver".to_owned())
        .build()?];

    let tempdir = TempDir::new()?;
    let include_path = tempdir.path().join("include");

    let mut handles = Vec::with_capacity(headers.len());

    fs::create_dir_all(&include_path)?;

    for header in headers {
        let dir = include_path.clone();

        handles.push(tokio::spawn(async move {
            write_archive_to_path(&dir, get_zip(&header.get_url()).await?)?;

            std::result::Result::<(), anyhow::Error>::Ok(())
        }));
    }

    for handle in handles {
        handle.await??;
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
        .header(
            include_path
                .join("rev/CANSparkMaxDriver.h")
                .to_str()
                .unwrap(),
        )
        .allowlist_type("c_(SparkMax|REVLib)_.*")
        .allowlist_function("c_(SparkMax|REVLib)_.*")
        .allowlist_var("c_(SparkMax|REVLib)_.*")
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

    let mut handles = Vec::with_capacity(libs.len());

    for lib in libs {
        let libs_dir = libs_dir.clone();

        handles.push(tokio::spawn(async move {
            let mut archive = get_zip(&lib.get_url()).await?;

            let mut zip_file = lib.find_lib_in_zip(&mut archive)?;

            let mut fs_file =
                File::create(libs_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

            std::io::copy(&mut zip_file, &mut fs_file)?;

            println!("cargo:rustc-link-lib=dylib={}", lib.get_lib_name().unwrap());

            std::result::Result::<(), anyhow::Error>::Ok(())
        }));
    }

    for handle in handles {
        handle.await??;
    }

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
