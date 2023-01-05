use crate::{
    utils::Valeriyya,
    Context, Error,
};

/// Puts the current song on repeat.
#[poise::command(
    prefix_command,
    slash_command,
    default_member_permissions = "VIEW_CHANNEL",
    rename = "loop",
    category = "Music"
)]
pub async fn loop_music(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let handler = handler_lock.lock().await;
        if handler.queue().current().is_some() {
            handler.queue().current().unwrap().enable_loop();

            ctx.send(Valeriyya::reply_default().embed(
                Valeriyya::embed()
                    .description("Loop enabled")
                    .title("Loop information")
            )).await;
        } else {
            ctx.send(Valeriyya::reply_default().embed(
                Valeriyya::embed()
                    .description("There is no songs in the queue.")
                    .title("Loop information")
            )).await;
        }
    };

    Ok(())
}
