use poise::serenity_prelude::Mentionable;
use crate::{
    serenity,
    utils::{get_guild_db, update_guild_db},
    Context, Error,
};

#[derive(poise::ChoiceParameter, Debug)]
pub enum ChannelTypeChoices {
    #[name = "logs"]
    Logs,
    #[name = "welcome"]
    Welcome,
}

/// Changes the settings in this guild.
#[poise::command(slash_command, category = "Settings", subcommands("channel", "role"), default_member_permissions="MANAGE_GUILD")]
pub async fn settings(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// The channel that will be used for the previous type.
#[poise::command(slash_command, category = "Settings")]
pub async fn channel(
    ctx: Context<'_>,
    #[description = "Select a channel setting."]
    #[rename = "type"]
    type_option: ChannelTypeChoices,
    #[description = "The channel that will be used for the previous type."]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let database = &ctx.data().database;

    let guild_id = ctx.guild_id().unwrap().0;

    let mut db = get_guild_db(database, guild_id).await;
    if let ChannelTypeChoices::Logs = type_option {
        db.channels.logs = Some(channel.id.to_string());
        ctx.say(format!("The logs channel has been updated to {}.", channel.mention())).await;
    } else {
        db.channels.welcome = Some(channel.id.to_string());
        ctx.say(format!("The welcome channel has been updated to {}.", channel.mention())).await;
    };

    update_guild_db(database, guild_id, &db).await;

    Ok(())
}

#[derive(poise::ChoiceParameter, Debug)]
pub enum RoleTypeChoices {
    #[name = "staff"]
    Staff,
    #[name = "mute"]
    Mute,
}

#[poise::command(slash_command, category = "Settings")]
pub async fn role(
    ctx: Context<'_>,
    #[description = "Select a role setting."]
    #[rename = "type"]
    type_option: RoleTypeChoices,
    #[description = "The role that will be used for the previous type."]
    role: serenity::Role,
) -> Result<(), Error> {

    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap().0;

    let mut db = get_guild_db(database, guild_id).await;
    if let RoleTypeChoices::Staff = type_option {
        db.roles.staff = Some(role.id.to_string());
        ctx.say(format!("The staff role has been updated to {}.", role.mention())).await;
    };

    update_guild_db(database, guild_id, &db).await;

    Ok(())
}

