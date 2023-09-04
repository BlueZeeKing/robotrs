use std::sync::OnceLock;

use reqwest::Client;

pub mod artifact;
pub mod zip;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}
