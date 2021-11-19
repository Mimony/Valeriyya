#![feature(fn_traits)]
#![allow(unused_must_use)]

mod application;

pub use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    model::{
        channel::Message,
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content == "!ping" {
            message.reply(ctx, "Pong!").await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Ping pong".to_string(),
                _ => "Error".to_string()
            };

            if let Err(msg) = command
                .create_interaction_response(ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Error while respond to a interaction command: {}", msg);
            }
        }
    }

    async fn ready(&self, ctx: Context, client: Ready) {
        application::load_dev_guild_commands(ctx).await;
        
        println!("{} is online", client.user.name);
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder("OTA5NzkxNDU0MDQwMzAxNTY4.YZJbUQ.c8PIUM_EftouBg9KKV9bDG6IWCY")
        .intents(GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES)
        .event_handler(Handler)
        .application_id(909791454040301568)
        .await
        .unwrap();

    if let Err(msg) = client.start().await {
        println!("Client error: {:?}", msg);
    }
}