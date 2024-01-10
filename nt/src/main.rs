use std::time::Duration;

use nt_rs::{
    backends::tokio::TokioBackend,
    publish::Publisher,
    subscribe::Subscriber,
    time::init_time,
    types::{BinaryData, Properties, SubscriptionOptions},
    NetworkTableClient,
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

        let publish: Publisher<i32> = client.publish("/SmartDashboard/val".to_owned()).unwrap();

        publish.set(8).unwrap();

        main.await;

        // main.abort(); // End the main recieve loop
    }

    task.await.unwrap(); // Wait for the backend to stop
}
