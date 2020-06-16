#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
use tokio::prelude::*;
use tokio::sync::oneshot;

use tokio::sync::mpsc::{channel};

mod time;
use time::{sleep};

mod message_generator;
use message_generator::{message_generator, Message};

mod file_sink;
use file_sink::{file_sink};

use failure::{Fallible};

#[tokio::main]
async fn main() -> Fallible<()>{
    let (tx, rx) = channel::<Message>(10);
    let (mut ctx, crx) = channel::<message_generator::Ctrl>(10);
    tokio::spawn(message_generator(crx, tx));
    tokio::spawn(file_sink(rx));
    sleep(2000).await;

    println!("health message sent....");
    let (rtx, rrx) = oneshot::channel();
    ctx.send(message_generator::Ctrl::Health(rtx)).await?;
    println!("Received response healthy");

    sleep(2000).await;

    println!("Quit msg sent.....");
    ctx.send(message_generator::Ctrl::Quit).await?;

    Ok(())
}
