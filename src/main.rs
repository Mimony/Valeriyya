#![feature(fn_traits)]
#![feature(once_cell)]
#![allow(unused_must_use)]
#![allow(clippy::uninlined_format_args)]

mod commands;
mod utils;

use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Database};
use poise::serenity_prelude::GatewayIntents;
use poise::serenity_prelude::FullEvent;
use tracing::{error, info};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    db_client: Client,
    api_key: String,
    songbird: std::sync::Arc<songbird::Songbird>
}

impl Data {
    pub fn database(&self) -> Database {
        self.db_client.database("Valeriyya")
    }
}

fn event_listeners(
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {

    #[allow(clippy::single_match)]
    match event {
        FullEvent::Ready { ctx, data_about_bot } => {
            ctx.online();
            info!("{} is connected!", data_about_bot.user.tag())
        },
        _ => {},
    }

    Ok(())
}

async fn init() -> Result<(), Error> {
    tracing_subscriber::fmt().pretty().init();

    let discord_token =
        std::env::var("VALERIYYA_DISCORD_TOKEN").expect("(DISCORD_TOKEN IS NOT PRESENT)");
    let database_url = std::env::var("VALERIYYA_MONGODB").expect("(MONGODB_TOKEN IS NOT PRESENT)");
    let api_key = std::env::var("VALERIYYA_API_KEY").expect("(API_TOKEN IS NOT PRESENT)");

    let songbird = songbird::Songbird::serenity();

    let database_options =
        ClientOptions::parse_with_resolver_config(database_url, ResolverConfig::cloudflare())
            .await?;
    let db_client = Client::with_options(database_options)?;
    let discord_intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let options = poise::FrameworkOptions {
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
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".to_string()),
            ..Default::default()
        },
        listener: |event, framework, data| {
            Box::pin(async move {
                event_listeners(event, framework, data)
            })
        },
        ..Default::default()
    };

    let data = Data {
        db_client,
        api_key,
        songbird: songbird.clone()
    };

    let framework = poise::Framework::new(options, move |_ctx, _ready, _framework| {
        Box::pin(async {
            Ok(data)
        })
    });

    let mut client = poise::serenity_prelude::Client::builder(discord_token, discord_intents)
        .voice_manager_arc(songbird)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = init().await {
        error!("{}", e);
        std::process::exit(1);
    }
}
