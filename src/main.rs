#![feature(fn_traits)]
#![feature(once_cell)]
#![allow(unused_must_use)]

mod commands;
mod utils;

use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Database};
use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Data {
    client_id: serenity::UserId,
    db_client: Client,
    database: Database,
    api_key: String,
}

async fn event_listeners(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    #[allow(clippy::single_match)]
    match event {
        poise::Event::Ready {
            data_about_bot: bot,
        } => {
            println!("{} is connected!", bot.user.name)
        }
        _ => {}
    }

    Ok(())
}

async fn init() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let discord_token = match std::env::var("VALERIYYA-DEVELOP-MODE").unwrap() == "true" {
        true => std::env::var("VALERIYYA-DISCORD-DEV-TOKEN").unwrap(),
        false => std::env::var("VALERIYYA-DISCORD-TOKEN").unwrap(),
    };
    let database_url = std::env::var("VALERIYYA-MONGODB").unwrap();
    let api_key = std::env::var("VALERIYYA-API-KEY").unwrap();

    let database_options =
        ClientOptions::parse_with_resolver_config(database_url, ResolverConfig::cloudflare())
            .await?;
    let db_client = Client::with_options(database_options)?;
    let database = db_client.database("Valeriyya");

    let options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".to_string()),
            ..Default::default()
        },
        commands: vec![
            // Information Commands
            commands::info::user(),
            commands::info::help(),
            commands::info::register(),
            // Music Commands
            commands::music::play(),
            commands::music::skip(),
            commands::music::leave(),
            commands::music::join(),
            commands::music::loop_music(),
            // Moderation Commands
            commands::moderation::ban(),
            commands::moderation::kick(),
            commands::moderation::mute(),
            commands::moderation::cases(),
            commands::moderation::reference(),
            commands::moderation::reason(),
            // Settings Command
            commands::settings::settings(),
        ],
        listener: |ctx, event, framework, user_data| {
            Box::pin(event_listeners(ctx, event, framework, user_data))
        },
        ..Default::default()
    };

    let client = poise::Framework::builder()
        .token(discord_token)
        .user_data_setup(move |_ctx, client, _framework| {
            Box::pin(async move {
                Ok(Data {
                    db_client,
                    database,
                    client_id: client.user.id,
                    api_key
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged()
                | serenity::GatewayIntents::GUILD_MEMBERS
                | serenity::GatewayIntents::GUILD_MESSAGES
                | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .client_settings(move |c| c.register_songbird());

    client.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = init().await {
        println!("{}", e);
        std::process::exit(1);
    }
}
