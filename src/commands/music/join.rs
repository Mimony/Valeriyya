use crate::{Context, Error, utils::Valeriyya};

#[doc = "Joins a voice channel."]
#[poise::command(prefix_command, slash_command, default_member_permissions="VIEW_CHANNEL", category="Music")]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;
    
    let channel_id = guild
    .voice_states
    .get(&ctx.author().id)
    .and_then(|voice_state| voice_state.channel_id);
    
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send(Valeriyya::reply("Not in a voice channel").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let _ = ctx.data().songbird.join(guild_id, connect_to).await;

    ctx.send(Valeriyya::reply("I have joined your channel").ephemeral(true)).await?;

    Ok(())
}