use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::{Context, Error, utils::PURPLE_COLOR};

/// Leaves the voice channel.
#[poise::command(
    prefix_command,
    slash_command,
    default_member_permissions = "VIEW_CHANNEL", 
    category="Music"
)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await;
        ctx.send(CreateReply::default() 
            .embed(CreateEmbed::default() 
                .color(PURPLE_COLOR)
                    .description("Leaving the current channel.")
                    .title("Left the channel")
                    .timestamp(poise::serenity_prelude::Timestamp::now())
            )
        )
        .await;
    } else {
        ctx.send(CreateReply::default()
            .embed(CreateEmbed::default()
                .color(PURPLE_COLOR)
                    .description("I need to be in a voice channel to be able to leave.")
                    .title("Error")
                    .timestamp(poise::serenity_prelude::Timestamp::now())
            )
        )
        .await;
    }

    Ok(())
}
