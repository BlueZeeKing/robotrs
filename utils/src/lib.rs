use std::sync::OnceLock;

use reqwest::Client;
use serde::{Deserialize, Serialize};

static CLIENT: OnceLock<Client> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    #[serde(rename = "groupId")]
    group_id: String,
    #[serde(rename = "artifactId")]
    artifact_id: String,
    version: String,
    versioning: Versioning,
}

#[derive(Serialize, Deserialize, Debug)]
struct Versioning {
    latest: String,
    release: String,
}

fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

async fn get_metadata(main_name: &str, secondary_name: &str) -> anyhow::Result<Metadata> {
    let response = get_client().get(format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{main_name}/{secondary_name}/maven-metadata.xml")).header(reqwest::header::ACCEPT, "text/xml").send().await?;

    let data = response.bytes().await?;

    let data: Metadata = serde_xml_rs::from_reader(&*data)?;

    Ok(data)
}

pub async fn get_artifact_url(
    main_name: &str,
    secondary_name: &str,
    artifact_name: fn(&str) -> String,
) -> anyhow::Result<String> {
    let metadata = get_metadata(main_name, secondary_name).await?;

    Ok(format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{main_name}/{secondary_name}/{}/{}", metadata.versioning.release, artifact_name(&metadata.versioning.release)))
}
