use megalodon::{generator, streaming::Message};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return;
    };

    streaming(url.as_str()).await;
}

async fn streaming(url: &str) {
    let client = generator(megalodon::SNS::Mastodon, url.to_string(), None, None).unwrap();
    let streaming = client.local_streaming().await;

    streaming
        .listen(Box::new(|message| {
            Box::pin({
                async move {
                    match message {
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
                    }
                }
            })
        }))
        .await;
}
