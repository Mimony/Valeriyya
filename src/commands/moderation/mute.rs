use crate::{
    serenity,
    utils::{create_case, get_guild_db, member_managable, string_to_sec, ActionTypes, Case},
    Context, Error,
};
use poise::serenity_prelude::Timestamp;

/// Mutes a member for a specified time.
#[poise::command(prefix_command, slash_command, category = "Moderation", default_member_permissions="MODERATE_MEMBERS")]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The member to mute"] mut member: serenity::Member,
    #[description = "The time the member to be muted for. (Max 28 days)."] time: String,
    #[description = "The reason for this mute."] #[rest] reason: Option<String>,
) -> Result<(), Error> {
    let reason_default = reason.unwrap_or_else(|| String::from("default reason"));
    let string_time = string_to_sec(&time);
    
    if string_time < 60 {
        ctx.send(|m| {
            m.content("You can't mute someone for under 60 seconds!")
            .ephemeral(true)
        }).await;
        return Ok(());
    }

    let timestamp = Timestamp::from_unix_timestamp(
        Timestamp::unix_timestamp(&Timestamp::now()) + string_time,
    )
    .ok();

    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    let db = get_guild_db(database, guild_id.0).await;

    if !member_managable(ctx, &member).await {
        ctx.send(|m| {
            m.content("The member can't be managed so you can't mute them!")
                .ephemeral(true)
        })
        .await;
        return Ok(());
    }
    
    if member.communication_disabled_until.is_some() & ((Timestamp::unix_timestamp(&Timestamp::now()) - member.communication_disabled_until.unwrap().unix_timestamp()) < 0) {
        ctx.send(|m| m.content("This member is already muted").ephemeral(true))
            .await;
        return Ok(());
    };

    member
        .disable_communication_until_datetime(&ctx.discord().http, timestamp.unwrap())
        .await;
    create_case(
        database,
        ctx.guild_id().unwrap().0,
        Case {
            id: db.cases_number + 1,
            action: ActionTypes::mute,
            guild_id: guild_id.0.to_string(),
            staff_id: ctx.author().id.to_string(),
            target_id: member.user.id.to_string(),
            date: Timestamp::unix_timestamp(&Timestamp::now()),
            reason: reason_default.to_string(),
            expiration: Some(timestamp.unwrap().unix_timestamp()),
            reference: None,
        },
    )
    .await;
    ctx.send(|s| {
        s.embed(|e| {
            e.color(serenity::Color::from_rgb(82, 66, 100))
                .author(|a| {
                    a.name(format!("{} ({})", member.user.tag(), member.user.id))
                        .icon_url(ctx.author().face())
                })
                .thumbnail(ctx.guild().unwrap().icon_url().unwrap())
                .description(format!(
                    "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration: {}",
                    member.user.tag(),
                    ActionTypes::mute,
                    reason_default,
                    time_format(time)
                ))
                .timestamp(Timestamp::now())
        })
        .ephemeral(true)
    })
    .await;

    Ok(())
}

fn time_format(time: String) -> String {
    format!(
        "<t:{}:R>",
        Timestamp::unix_timestamp(&Timestamp::now()) + string_to_sec(time)
    )
}
