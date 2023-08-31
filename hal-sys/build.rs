use utils;

#[tokio::main]
async fn main() {
    utils::build().await.unwrap();
}
