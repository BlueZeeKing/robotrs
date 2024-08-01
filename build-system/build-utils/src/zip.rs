use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, Cursor},
    path::Path,
};

use bytes::Bytes;
use zip::ZipArchive;

use super::get_client;

pub async fn get_zip(url: &str) -> anyhow::Result<ZipArchive<Cursor<Bytes>>> {
    let bytes = get_client()
        .get(url)
        .header(reqwest::header::ACCEPT, "application/zip")
        .send()
        .await?
        .bytes()
        .await?;

    Ok(ZipArchive::new(Cursor::new(bytes))?)
}

pub fn write_archive_to_path(
    path: &Path,
    mut archive: ZipArchive<Cursor<Bytes>>,
) -> anyhow::Result<()> {
    for index in 0..archive.len() {
        let mut zip_file = archive.by_index(index)?;

        let file_path = path.join(
            zip_file
                .enclosed_name()
                .ok_or(anyhow::anyhow!("Could not get file name"))?,
        );

        if zip_file.is_dir() {
            fs::create_dir_all(file_path)?;
            continue;
        }

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut fs_file = File::create(file_path)?;

        io::copy(&mut zip_file, &mut fs_file)?;
    }

    Ok(())
}

pub fn extract_libs(
    archive: &mut ZipArchive<Cursor<Bytes>>,
) -> anyhow::Result<Vec<(String, usize)>> {
    let mut res = Vec::new();

    for zip_index in 0..archive.len() {
        let file = archive.by_index(zip_index)?;
        let Some(file_path) = file.enclosed_name() else {
            continue;
        };

        let Some(prefix) = file_path.file_prefix().and_then(OsStr::to_str) else {
            continue;
        };

        let Some(file_name) = file_path.file_name().and_then(OsStr::to_str) else {
            continue;
        };

        if !prefix.starts_with("lib") || file_name.to_lowercase().contains("debug") {
            continue;
        }

        res.push((prefix.trim_start_matches("lib").to_string(), zip_index));
    }

    Ok(res)
}
