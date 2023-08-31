use utils;

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    utils::build().await.unwrap();
}
