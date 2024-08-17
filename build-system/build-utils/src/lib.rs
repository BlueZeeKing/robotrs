#![feature(path_file_prefix)]

use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::bail;
use artifact::{is_robot, Artifact};
use reqwest::Client;
use tempfile::TempDir;

use crate::zip::{get_zip, write_archive_to_path};

pub mod artifact;
pub mod zip;

static CLIENT: OnceLock<Client> = OnceLock::new();
pub const WPI_VERSION: &str = "2024.3.2";

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(Client::new)
}

pub async fn gen_bindings(
    artifacts: &[Artifact],
    allow: &'static str,
    path: &[&Path],
    out_path: &Path,
) -> anyhow::Result<()> {
    let tempdir = TempDir::new()?;
    let include_path = tempdir.path().join("include");

    fs::create_dir_all(&include_path)?;

    for header in artifacts.iter().filter(|artifact| artifact.has_headers()) {
        write_archive_to_path(&include_path, get_zip(&header.get_header_url()).await?)?;
    }

    if let Ok(host) = env::var("HOST") {
        env::set_var("TARGET", host);
    }

    let mut builder = bindgen::Builder::default().clang_args([
        "-xc++",
        "-std=c++20",
        &format!("--include-directory={}", include_path.to_str().unwrap()),
    ]);

    for header in path {
        builder = builder.header(include_path.join(header).to_str().unwrap());
    }

    let result = builder
        .allowlist_type(allow)
        .allowlist_function(allow)
        .allowlist_var(allow)
        .generate()?;

    result.write_to_file(out_path)?;

    Ok(())
}

pub async fn build(artifacts: &[Artifact]) -> anyhow::Result<()> {
    let Some(libs_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
        bail!("Unable to find libs dir");
    };

    let out_dir = env::var("LIBS_OUT_DIR")
        .map(PathBuf::from)
        .expect("Could not find out dir, make sure to set it in the cargo config file");

    fs::create_dir_all(&libs_dir)?;
    fs::create_dir_all(&out_dir)?;

    println!(
        "cargo:rustc-link-search=native={}",
        libs_dir.to_str().unwrap()
    );

    for lib in artifacts
        .iter()
        .filter(|artifact| artifact.get_lib_name().is_some())
        .filter(|artifact| (artifact.is_robot_only() && is_robot()) || !artifact.is_robot_only())
    {
        dbg!(lib.is_robot_only());
        dbg!(lib.get_lib_url());
        let mut archive = get_zip(&lib.get_lib_url()).await?;

        let mut zip_file = lib.find_lib_in_zip(&mut archive)?;

        let mut fs_file =
            File::create(libs_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

        std::io::copy(&mut zip_file, &mut fs_file)?;

        if lib.should_deploy() {
            let mut fs_file =
                File::open(libs_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

            let mut out_file =
                File::create(out_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

            std::io::copy(&mut fs_file, &mut out_file)?;
        }

        println!("cargo:rustc-link-lib=dylib={}", lib.get_lib_name().unwrap());
    }

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
