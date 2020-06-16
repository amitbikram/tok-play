use tokio::sync::mpsc::{Receiver};
use crate::message_generator::{Message};

pub async fn file_sink(mut channel: Receiver<Message>) {
    while let Some(msg) = channel.recv().await {
        println!("Writing to file {:?}", msg);
    }
}