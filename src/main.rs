#![feature(fn_traits)]
#![allow(unused_must_use)]
#![allow(unused_macros)]


mod commands;


use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    bot_user_id: serenity::UserId,
    bot_start_time: std::time::Instant,
    // database: sqlx::Postgres,
}

async fn app() -> Result<(), Error> {
    let discord_token = "OTA5NzkxNDU0MDQwMzAxNTY4.YZJbUQ.c8PIUM_EftouBg9KKV9bDG6IWCY";

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::info::user(),
        ],
        ..Default::default()
    };

    poise::Framework::build()
        .token(discord_token)
        .user_data_setup(move |ctx, bot, _framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::playing("development")).await;
                Ok(Data {
                    bot_user_id: bot.user.id,
                    bot_start_time: std::time::Instant::now(),
                })
            })
        })
        .options(options)
        .client_settings(move |client_builder| {
            client_builder.intents(
                serenity::GatewayIntents::non_privileged()
                    | serenity::GatewayIntents::GUILD_MEMBERS,
            )
        })
        .run()
        .await?;

    Ok(())
}
#[tokio::main]
async fn main() {
    if let Err(e) = app().await {
        println!("{}", e);
        std::process::exit(1);
    }
}