use std::{
    env,
    fs::{self, File},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::{bail, Result};
use bytes::Bytes;
use reqwest::Client;
use zip::ZipArchive;

#[tokio::main]
async fn main() {
    build().await.unwrap();

    cxx_build::bridge("src/main.rs").compile("ctre-rs");

    println!("cargo:rustc-link-lib=ctre-cpp-wrapper");

    println!("cargo:rustc-link-lib=ctre-rs");
}

const LIBS: &[(&str, &str, &str, &str)] = &[
    (
        "phoenix/api-cpp",
        "api-cpp-$VERSION-linuxathena.zip",
        "CTRE_Phoenix",
        "5.30.4",
    ),
    (
        "phoenix/cci",
        "cci-$VERSION-linuxathena.zip",
        "CTRE_PhoenixCCI",
        "5.30.4",
    ),
    (
        "phoenixpro/tools",
        "tools-$VERSION-linuxathena.zip",
        "CTRE_PhoenixTools",
        "23.0.10",
    ),
];

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

fn get_artifact_url(name: &str, artifact_name: &str, version: &str) -> String {
    format!(
        "https://maven.ctr-electronics.com/release/com/ctre/{name}/{}/{}",
        version, artifact_name
    )
}

async fn build() -> Result<()> {
    download_libs().await?;
    shoutout_libs().await?;

    Ok(())
}

async fn download_libs() -> Result<()> {
    // let Some(libs_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
    //     bail!("Unable to find out dir");
    // };

    let libs_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("libs");

    fs::create_dir_all(&libs_dir)?;

    for (name, artifact, lib, version) in LIBS {
        let url = get_artifact_url(name, &artifact.replace("$VERSION", &version), version);

        let mut zip = get_zip(&url).await?;

        for file_index in 0..zip.len() {
            let mut file = zip.by_index(file_index)?;

            let path = file.enclosed_name().unwrap();
            let Some(file_name) = path.file_name().map(|str| str.to_str().unwrap()) else {
                continue;
            };

            if file_name.starts_with("lib") && !file_name.ends_with("debug") {
                let mut index = file_name.len() - 1;
                let chars = file_name.chars().collect::<Vec<_>>();
                for (idx, char) in chars.iter().enumerate().rev() {
                    if *char == 'o' && chars[idx - 1] == 's' {
                        index = idx;
                        break;
                    }
                }
                let file_name = &file_name[..index + 1];

                if &file_name.trim_start_matches("lib").trim_end_matches(".so") == lib {
                    let mut lib_file = File::create(libs_dir.join(file_name))?;

                    let mut file_data = Vec::new();
                    file.read_to_end(&mut file_data)?;

                    lib_file.write_all(&file_data)?;

                    break;
                }
            }
        }
    }

    Ok(())
}

async fn shoutout_libs() -> Result<()> {
    // let Some(out_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
    //     bail!("Unable to find out dir");
    // };

    let libs_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("libs");

    println!(
        "cargo:rustc-link-search=native={}",
        libs_dir.to_str().unwrap()
    );

    for (_, _, lib, _) in LIBS {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    Ok(())
}

async fn get_zip(url: &str) -> Result<ZipArchive<Cursor<Bytes>>> {
    let bytes = get_client()
        .get(url)
        .header(reqwest::header::ACCEPT, "application/zip")
        .send()
        .await?
        .bytes()
        .await?;

    Ok(ZipArchive::new(Cursor::new(bytes))?)
}
