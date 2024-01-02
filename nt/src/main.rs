use nt_rs::{time::init_time, tokio::TokioBackend, NetworkTableClient, Publisher};

#[tokio::main]
async fn main() {
    init_time();

    let (client, task) = NetworkTableClient::new::<TokioBackend>("localhost", "rust")
        .await
        .unwrap();

    {
        let client = client;

        let main = tokio::spawn(client.main_task::<TokioBackend>());

        let publisher: Publisher<String> = client
            .publish("/FMSInfo/GameSpecificMessage".to_owned())
            .unwrap();

        publisher.set("Hello, World!".to_owned()).unwrap();

        main.abort(); // End the main recieve loop
    }

    task.await.unwrap(); // Wait for the backend to stop
}
