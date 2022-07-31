#![feature(fn_traits)]
#![feature(once_cell)]
#![allow(unused_must_use)]

mod commands;
mod utils;

use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Database};

use poise::serenity_prelude::{self as serenity, Action, MemberAction, Timestamp};

use crate::utils::{create_case, get_guild_db, ActionTypes, Case};
use songbird::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Data {
    client_id: serenity::UserId,
    db_client: Client,
    database: Database,
}

async fn event_listeners(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready {
            data_about_bot: bot,
        } => {
            println!("{} is connected!", bot.user.name)
        }
        poise::Event::GuildMemberRemoval {
            guild_id: gid,
            user,
            member_data_if_available: _member,
        } => {
            let audit_logs = gid.audit_logs(ctx, None, None, None, None).await?;
            let audit_logs_latest = audit_logs
                .entries
                .iter()
                .find(|u| u.target_id.unwrap().0 == user.id.0)
                .unwrap();

            if let Action::Member(MemberAction::Kick) = audit_logs_latest.action {
                let db = get_guild_db(&user_data.database, gid.0.to_string()).await;
                create_case(
                    &user_data.database,
                    gid.0.to_string(),
                    Case {
                        id: db.cases_number + 1,
                        action: ActionTypes::kick,
                        guild_id: gid.0.to_string(),
                        staff_id: audit_logs_latest.user_id.0.to_string(),
                        target_id: audit_logs_latest.target_id.unwrap().to_string(),
                        reason: audit_logs_latest
                            .reason
                            .clone()
                            .unwrap_or_else(|| "Default reason".to_string()),
                        date: Timestamp::unix_timestamp(&Timestamp::now()),
                        expiration: None,
                        reference: None,
                    },
                )
                .await;
            };
        },
        _ => {}
    }

    Ok(())
}

async fn init() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let discord_token = std::env::var("VALERIYYA-DISCORD-TOKEN").unwrap();
    let database_url = std::env::var("VALERIYYA-MONGODB").unwrap();

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

    let client = poise::Framework::build()
        .token(discord_token)
        .user_data_setup(move |ctx, client, _framework| {
            Box::pin(async move {
                ctx.set_activity(Some(poise::serenity::gateway::ActivityData {
                    name: String::from("the lovely moon"),
                    kind: serenity::model::gateway::ActivityType::Watching,
                    url: None
                })).await;
                Ok(Data {
                    db_client,
                    database,
                    client_id: client.user.id,
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
        .client_settings(move |c| {
            c.register_songbird()
        });


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
