use std::time::Duration;

use nt_rs::{
    backends::tokio::TokioBackend,
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

        let mut subscriber: Subscriber<BinaryData> = client
            .subscribe(
                "/SmartDashboard".to_owned(),
                SubscriptionOptions::default().prefix(true),
            )
            .unwrap();

        loop {
            if let Ok(mut child) = subscriber.get_child::<BinaryData>().await {
                dbg!(child.name());
                dbg!(child.get().await.unwrap());
            } else {
                break;
            }
        }

        main.abort(); // End the main recieve loop
    }

    task.await.unwrap(); // Wait for the backend to stop
}
