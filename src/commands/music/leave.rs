use crate::{utils::Valeriyya, Context, Error};

#[doc = "Leaves the voice channel."]
#[poise::command(
    prefix_command,
    slash_command,
    default_member_permissions = "VIEW_CHANNEL",
    category = "Music"
)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if ctx.data().songbird.get(guild_id).is_some() {
        ctx.data().songbird.remove(guild_id).await;

        ctx.send(Valeriyya::reply_default().embed(
            Valeriyya::embed()
                .description("Leaving the current channel.")
                .title("Left the channel")
        )).await;
    } else {
        ctx.send(Valeriyya::reply_default().embed(
            Valeriyya::embed()
                .description("I need to be in a voice channel to be able to leave.")
                .title("Error")
        )).await;
    }

    Ok(())
}
