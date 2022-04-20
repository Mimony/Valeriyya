#![feature(fn_traits)]
#![feature(once_cell)]
#![allow(unused_must_use)]

mod commands;
mod utils;
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{Client, Database};
use poise;
use poise::serenity_prelude::{self as serenity, Action, MemberAction, Timestamp};

use crate::utils::{get_guild_db, Case, create_case, ActionTypes};

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
    _framework: &poise::Framework<Data, Error>,
    user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot: bot } => {
            println!("{} is connected!", bot.user.name)
        },
        poise::Event::GuildMemberRemoval { guild_id: gid, user, member_data_if_available: _member } => {
            let audit_logs = gid.audit_logs(ctx, None, None, None, None).await?;
            let audit_logs_latest = audit_logs.entries.iter().find(|u| u.target_id.unwrap() == user.id.0).unwrap();

            if let Action::Member(MemberAction::Kick) = audit_logs_latest.action {
                let db = get_guild_db(&user_data.database, gid.0.to_string()).await;
                create_case(&user_data.database, gid.0.to_string(), Case {
                    id: db.cases_number + 1,
                    action: ActionTypes::kick,
                    guild_id: gid.0.to_string(),
                    staff_id: audit_logs_latest.user_id.0.to_string(),
                    target_id: audit_logs_latest.target_id.unwrap().to_string(),
                    reason: audit_logs_latest.reason.clone().unwrap_or("Default reason".to_string()),
                    date: Timestamp::unix_timestamp(&Timestamp::now()),
                    expiration: None
                }).await;
            };
        },
        _ => {}
    }

    Ok(())
}

async fn init() -> Result<(), Error> {
    let discord_token = "ODMwMTMwMzAxNTM1NjQ5ODUz.YHCNFg.mYuhKgA5WQWAP71BsCxT8pMjJCQ";
    let database_url = "mongodb+srv://Client:MomsSpaghetti@cluster0.i1oux.mongodb.net/myFirstDatabase?retryWrites=true&w=majority";

    let database_options =
        ClientOptions::parse_with_resolver_config(&database_url, ResolverConfig::cloudflare())
            .await?;
    let db_client = Client::with_options(database_options)?;
    let database = db_client.database("Valeriyya");

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::info::user(),
            commands::moderation::ban(),
            commands::moderation::kick(),
            commands::moderation::mute(),
            commands::moderation::cases(),
            poise::Command {
                subcommands: vec![commands::settings::channel(), commands::settings::role()],
                ..commands::settings::settings()
            },
        ],
        listener: |ctx, event, framework, user_data| {
            Box::pin(event_listeners(ctx, event, framework, user_data))
        },
        ..Default::default()
    };

    poise::Framework::build()
        .token(discord_token)
        .user_data_setup(move |ctx, client, _framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::watching("the lovely moon"))
                    .await;
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
                | serenity::GatewayIntents::GUILD_MESSAGES,
        )
        .run()
        .await?;
    Ok(())
}
#[tokio::main]
async fn main() {
    if let Err(e) = init().await {
        println!("{}", e);
        std::process::exit(1);
    }
}
