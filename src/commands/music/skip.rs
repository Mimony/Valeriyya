use poise::CreateReply;

use crate::{Context, Error};

/// Skips the currently playing song.
#[poise::command(prefix_command, slash_command, category="Music", aliases("s"))]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    
    let channel_id = ctx.guild().unwrap()
    .voice_states
    .get(&ctx.author().id)
    .and_then(|voice_state| voice_state.channel_id);
    
    let _connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send(CreateReply::default().content("You are not in a voice channel").ephemeral(true))
            .await?;
            
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
                drop(handler);
                ctx.send(CreateReply::default().content("There is no songs in the queue!")
                    .ephemeral(true)
                ).await;
            },
        };
    };

    Ok(())
}