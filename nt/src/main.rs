use nt::{backend::TokioBackend, time::init_time, NetworkTableClient};

#[tokio::main]
async fn main() {
    init_time();

    let (client, _) = NetworkTableClient::new::<TokioBackend>("localhost", "rust")
        .await
        .unwrap();

    client.main_loop::<TokioBackend>().await.unwrap();
}
