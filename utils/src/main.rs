use std::io::Cursor;

use reqwest::Client;
use utils::get_artifact_url;
use zip::ZipArchive;

#[tokio::main]
async fn main() {
    let url = get_artifact_url("hal", "hal-cpp", |version| {
        format!("hal-cpp-{version}-headers.zip")
    })
    .await
    .unwrap();

    let client = Client::new();

    let bytes = client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/zip")
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let zip = ZipArchive::new(Cursor::new(&bytes)).unwrap();

    dbg!(&zip);
}
