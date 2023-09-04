use crate::{
    structs::{GuildDb, GuildDbChannels, GuildDbRoles},
    Context, Error,
};

use poise::serenity_prelude::Mentionable;

#[derive(poise::ChoiceParameter, Debug)]
pub enum ChannelTypeChoices {
    #[name = "logs"]
    Logs,
    #[name = "welcome"]
    Welcome,
}

#[doc = "Changes the settings in this guild."]
#[poise::command(slash_command, category = "Settings", subcommands("channel", "role"), default_member_permissions="MANAGE_GUILD")]
pub async fn settings(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, category = "Settings")]
pub async fn channel(
    ctx: Context<'_>,
    #[description = "Select a channel setting."]
    #[rename = "type"]
    type_option: ChannelTypeChoices,
    #[description = "The channel that will be used for the previous type."]
    #[channel_types("Text")]
    channel: poise::serenity_prelude::GuildChannel,
) -> Result<(), Error> {
    let database = &ctx.data().database();
    let guild_id = ctx.guild_id().unwrap();

    let mut db = GuildDb::new(database, guild_id.to_string()).await;
    if let ChannelTypeChoices::Logs = type_option {
        db = db.set_channels(GuildDbChannels::default().set_logs_channel(Some(channel.id.to_string())));
        ctx.say(format!("The logs channel has been updated to {}.", channel.mention())).await?;
    } else if let ChannelTypeChoices::Welcome = type_option {
        db = db.set_channels(GuildDbChannels::default().set_welcome_channel(Some(channel.id.to_string())));
        ctx.say(format!("The welcome channel has been updated to {}.", channel.mention())).await?;
    };

    db.execute(database).await;

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
    role: poise::serenity_prelude::Role,
) -> Result<(), Error> {

    let database = &ctx.data().database();
    let guild_id = ctx.guild_id().unwrap().0;

    let mut db = GuildDb::new(database, guild_id.to_string()).await;
    if let RoleTypeChoices::Staff = type_option {
        db = db.set_roles(GuildDbRoles::default().set_staff_role(Some(role.id.to_string())));
        ctx.say(format!("The staff role has been updated to {}.", role.mention())).await?;
    };

    db.execute(database).await;

    Ok(())
}

