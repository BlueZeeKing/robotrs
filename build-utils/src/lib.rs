use std::sync::OnceLock;

use chashmap::CHashMap;
use reqwest::Client;
use tokio::sync::Semaphore;

pub mod artifact;
pub mod zip;

static CLIENT: OnceLock<Client> = OnceLock::new();
static URLS: OnceLock<CHashMap<String, Semaphore>> = OnceLock::new();

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

pub fn get_semaphores() -> &'static CHashMap<String, Semaphore> {
    URLS.get_or_init(|| CHashMap::new())
}
