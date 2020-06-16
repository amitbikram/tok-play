use tokio::sync::mpsc::{Sender, Receiver};
use tokio::select;

use crate::time::{sleep};

mod message;
pub use message::{Message};

#[derive(Debug)]
pub enum Ctrl {
    Quit,
    Health,
}

pub async fn message_generator(mut ctrl_channel: Receiver<Ctrl>, mut channel: Sender<Message>) {
    loop {
        select! {
            msg = channel.send(Message::Hello)  =>
                match msg{
                    Ok(_) => sleep(100).await,
                    Err(_) => {
                        eprintln!("error sending msg");
                        break;
                    }
                },
            ctl = ctrl_channel.recv() =>
                match ctl {
                    Some(Ctrl::Quit) => break,
                    None => break
                }
        }
    }




    loop {
        match channel.send(Message::Hello).await {
            Ok(_) => sleep(100).await,
            Err(_) => {
                eprintln!("error sending msg");
                break;
            }
        }
    }
}