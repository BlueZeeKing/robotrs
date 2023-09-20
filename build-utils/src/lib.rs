use std::sync::OnceLock;

use reqwest::Client;

pub mod artifact;
pub mod zip;

static CLIENT: OnceLock<Client> = OnceLock::new();
pub const WPI_VERSION: &str = "2023.4.3";

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}
