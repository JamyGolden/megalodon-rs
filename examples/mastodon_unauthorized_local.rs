use megalodon::{generator, streaming::Message};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_STREAMING_URL") else {
        println!("Specify MASTODON_STREAMING_URL!!");
        return;
    };

    streaming(url.as_str()).await;
}

async fn streaming(url: &str) {
    let client = generator(megalodon::SNS::Mastodon, url.to_string(), None, None);
    let streaming = client.local_streaming(url.to_string());

    streaming
        .listen(Box::new(|message| match message {
            Message::Update(mes) => {
                println!("{:#?}", mes);
            }
            Message::Notification(mes) => {
                println!("{:#?}", mes);
            }
            Message::Conversation(mes) => {
                println!("{:#?}", mes);
            }
            Message::Delete(mes) => {
                println!("message is deleted: {}", mes);
            }
            Message::StatusUpdate(mes) => {
                println!("updated: {:#?}", mes)
            }
            Message::Heartbeat() => {
                println!("heartbeat");
            }
        }))
        .await;
}
