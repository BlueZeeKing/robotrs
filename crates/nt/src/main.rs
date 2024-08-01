use std::{thread, time::Duration};

use nt::{Instance, Publisher, Subscriber};

fn main() {
    let server = Instance::default();
    server.start_server("nt.json");
    if server.is_starting() {
        let mut started = false;

        for _ in 0..3 {
            thread::sleep(Duration::from_millis(15));

            if !server.is_starting() {
                started = true;
                break;
            }
        }

        if !started {
            panic!("Failed to start server");
        }
    }

    let publisher: Publisher<String> = server.topic("/test/value").publish(Default::default());

    publisher.set("Test".to_string());

    let subscriber: Subscriber<String> = server.topic("/test/value").subscribe(Default::default());

    loop {
        dbg!(subscriber.get());
        thread::sleep(Duration::from_secs(1));
    }
}
