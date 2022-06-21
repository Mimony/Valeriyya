use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, category="Music", aliases("s"))]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
    .voice_states
    .get(&ctx.author().id)
    .and_then(|voice_state| voice_state.channel_id);

    let _connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send(|m| m.content("Not in a voice channel").ephemeral(true))
                .await;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        match queue.is_empty() {
            false => { 
                queue.skip();
            },
            true => {
                ctx.send(|m| {
                    m.content("There is no songs in the queue!")
                    .ephemeral(true)
                }).await;
            },
        };
    };

    Ok(())
}