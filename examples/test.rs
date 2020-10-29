use std::thread;
use std::time::Duration;
use redis::{self, cmd};

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let client = redis::Client::open(format!("redis://127.0.0.1:6379"))?;
    let clone_client = client.clone();
    // receive message from foo channel
    thread::spawn(move || {
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe("foo").unwrap();
        loop {
            let msg = pubsub.get_message().unwrap();
            let channel = msg.get_channel_name();
            let content: i32 = msg.get_payload().unwrap();
            println!("receive channel {}, message {}", channel, content);
            if content == 5 {
                // subscribe bar channel using the same pubsub
                println!("sub bar channel");
                pubsub.subscribe("bar").unwrap();
            }
        }
    });
    // send message to foo channel
    thread::sleep(Duration::from_secs(1));
    let mut clone_conn = clone_client.get_connection()?;
    let mut m = 10;
    while m > 0 {
        m -= 1;
        cmd("PUBLISH").arg("foo").arg(m).query::<i32>(&mut clone_conn)?;
    }
    cmd("PUBLISH").arg("bar").arg(0).query::<i32>(&mut clone_conn)?;
    thread::sleep(Duration::from_secs(20));
    Ok(())
}