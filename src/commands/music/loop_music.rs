use poise::{CreateReply, serenity_prelude::CreateEmbed};

use crate::{Context, Error, utils::PURPLE_COLOR};

/// Puts the current song on repeat.
#[poise::command(
    prefix_command,
    slash_command,
    default_member_permissions = "VIEW_CHANNEL",
    rename = "loop",
    category="Music"
)]
pub async fn loop_music(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.queue().current().is_some() {
           handler.queue().current().unwrap().enable_loop();
           ctx.send(CreateReply::default()
            .embed( CreateEmbed::default()
                .color(PURPLE_COLOR)
                    .description("The loop has been enabled.")
                    .title("Loop information")
                    .timestamp(poise::serenity_prelude::Timestamp::now())
            )
        ).await;
        } else {
            ctx.send(CreateReply::default()
                .embed(CreateEmbed::default()
                    .color(PURPLE_COLOR)
                        .description("There is no songs in the queue.")
                        .title("Loop information")
                        .timestamp(poise::serenity_prelude::Timestamp::now())
                )
            ).await;
        }
    };

    Ok(())
}