use nt_rs::{backends::tokio::TokioBackend, time::init_time, NetworkTableClient, Subscriber};

#[tokio::main]
async fn main() {
    init_time();

    let (client, task) = NetworkTableClient::new::<TokioBackend>("localhost", "rust")
        .await
        .unwrap();

    {
        let client = client;

        let main = tokio::spawn(client.main_task::<TokioBackend>());

        let mut subscriber: Subscriber<String> = client
            .subscribe("/FMSInfo/GameSpecificMessage".to_owned())
            .unwrap();

        loop {
            if let Ok(val) = subscriber.get().await {
                dbg!(val);
            } else {
                break;
            }
        }

        main.abort(); // End the main recieve loop
    }

    task.await.unwrap(); // Wait for the backend to stop
}
