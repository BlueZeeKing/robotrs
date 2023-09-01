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
use tempfile::TempDir;
use zip::ZipArchive;

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    build().await.unwrap();
}

const HEADERS: &[(&str, &str, &str, &str)] = &[
    ("hal", "hal-cpp", "hal-cpp-$VERSION-headers.zip", "2023.4.3"),
    (
        "wpiutil",
        "wpiutil-cpp",
        "wpiutil-cpp-$VERSION-headers.zip",
        "2023.4.3",
    ),
    (
        "wpimath",
        "wpimath-cpp",
        "wpimath-cpp-$VERSION-headers.zip",
        "2023.4.3",
    ),
];

const REV_HEADERS: &[(&str, &str, &str)] = &[(
    "REVLib-driver",
    "REVLib-driver-$VERSION-headers.zip",
    "2023.1.3",
)];

const LIBS: &[(&str, &str, &str, &str)] = &[(
    "REVLib-driver",
    "REVLib-driver-$VERSION-linuxathena.zip",
    "REVLibDriver",
    "2023.1.3",
)];

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

fn get_artifact_url(
    main_name: &str,
    secondary_name: &str,
    artifact_name: &str,
    version: &str,
) -> String {
    format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{main_name}/{secondary_name}/{}/{}", version, artifact_name)
}

fn get_rev_url(name: &str, artifact: &str, version: &str) -> String {
    format!(
        "https://maven.revrobotics.com/com/revrobotics/frc/{name}/{}/{}",
        version, artifact
    )
}

async fn get_headers(temp_dir: &TempDir) -> Result<()> {
    let include_dir = temp_dir.path().join("include");

    fs::create_dir_all(&include_dir)?;

    for url in HEADERS
        .iter()
        .map(|(main, second, artifact, version)| {
            get_artifact_url(
                main,
                second,
                &artifact.replace("$VERSION", version),
                version,
            )
        })
        .chain(REV_HEADERS.iter().map(|(name, artifact, version)| {
            get_rev_url(name, &artifact.replace("$VERSION", version), version)
        }))
    {
        eprintln!("{}", url);

        let mut zip = get_zip(&url).await?;

        for index in 0..zip.len() {
            let mut file = zip.by_index(index)?;

            if file.is_dir() {
                continue;
            }

            let mut buf = Vec::new();

            file.read_to_end(&mut buf)?;

            let file_path = include_dir.join(file.name());

            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = File::create(file_path)?;

            file.write_all(&buf)?;
        }
    }

    Ok(())
}

async fn build() -> Result<()> {
    let temp_dir = TempDir::new()?;

    get_headers(&temp_dir).await?;
    gen_bindings(&temp_dir).await?;

    download_libs().await?;
    shoutout_libs().await?;

    Ok(())
}

async fn download_libs() -> Result<()> {
    let Some(libs_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
        bail!("Unable to find out dir");
    };

    fs::create_dir_all(&libs_dir)?;

    for (name, artifact, lib, version) in LIBS {
        let url = get_rev_url(name, &artifact.replace("$VERSION", &version), version);

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
    let Some(out_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
        bail!("Unable to find out dir");
    };

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );

    for (_, _, lib, _) in LIBS {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    }

    Ok(())
}

async fn gen_bindings(temp_dir: &TempDir) -> Result<()> {
    let include_dir = temp_dir.path().join("include");

    if let Ok(host) = env::var("HOST") {
        env::set_var("TARGET", host);
    }

    let result = bindgen::Builder::default()
        .clang_args([
            "-xc++",
            "-std=c++20",
            &format!("--include-directory={}", include_dir.to_str().unwrap()),
        ])
        .header(
            include_dir
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

