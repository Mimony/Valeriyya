#![feature(fn_traits)]

pub use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready
    },
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        //
    }

    async fn ready(&self, _: Context, client: Ready) {
        println!("{} is online", client.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder("OTA5NzkxNDU0MDQwMzAxNTY4.YZJbUQ.c8PIUM_EftouBg9KKV9bDG6IWCY")
        .await
        .unwrap();

    if let Err(msg) = client.start().await {
        println!("Client error: {:?}", msg);
    }
}