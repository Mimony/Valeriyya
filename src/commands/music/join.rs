use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, default_member_permissions="VIEW_CHANNEL")]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    
    let channel_id = guild
    .voice_states
    .get(&ctx.author().id)
    .and_then(|voice_state| voice_state.channel_id);
    
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send(|m| m.content("Not in a voice channel").ephemeral(true))
            .await;
            
            return Ok(());
        }
    };
    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    manager.join(guild_id, connect_to).await;

    Ok(())
}