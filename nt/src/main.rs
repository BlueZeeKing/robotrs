use std::time::Duration;

use nt_rs::{
    backends::tokio::TokioBackend, publish::Publisher, time::init_time, NetworkTableClient,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    init_time();

    let (client, task) = NetworkTableClient::new::<TokioBackend>("localhost", "rust")
        .await
        .unwrap();

    {
        let client = client;

        let main = tokio::spawn(client.main_task::<TokioBackend>());

        let publisher: Publisher<i32> = client.publish("val".to_owned()).unwrap();

        publisher.set(1).unwrap();

        dbg!("First");

        sleep(Duration::from_secs(10)).await;

        dbg!("Second");

        publisher.set(2).unwrap();

        sleep(Duration::from_secs(10)).await;

        dbg!("Third");

        main.abort(); // End the main recieve loop
    }

    task.await.unwrap(); // Wait for the backend to stop
}
