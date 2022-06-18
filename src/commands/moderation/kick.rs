use poise::serenity_prelude::Timestamp;

use crate::{serenity, utils::{member_managable, get_guild_db, ActionTypes, create_case, Case}, Context, Error};

/// Kicks a member from the guild.
#[poise::command(prefix_command, slash_command, category = "Moderation", default_member_permissions="KICK_MEMBERS")]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member to kick"] member: serenity::Member,
    #[description = "The reason for this kick."] #[rest] reason: Option<String>,
) -> Result<(), Error> {
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap().0;

    let db = get_guild_db(database, guild_id).await;


    let reason_default = reason.unwrap_or_else(|| String::from("Default reason"));

    if !member_managable(ctx, &member).await {
        ctx.send(|m| {
            m.content("The member can't be managed so you can't kick them!")
            .ephemeral(true)
        }).await;
        return Ok(());
    }
        member.kick(ctx.discord()).await;
        create_case(
            database,
            ctx.guild_id().unwrap().0,
            Case {
                id: db.cases_number + 1,
                action: ActionTypes::kick,
                guild_id: guild_id.to_string(),
                staff_id: ctx.author().id.to_string(),
                target_id: member.user.id.to_string(),
                date: Timestamp::unix_timestamp(&Timestamp::now()),
                reason: reason_default.to_string(),
                expiration: None,
                reference: None,
            },
        ).await;
        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100))
                    .author(|a| {
                        a.name(format!("{} ({})", member.user.tag(), member.user.id));
                        a.icon_url(ctx.author().face())
                    })
                    .thumbnail(ctx.guild().unwrap().icon_url().unwrap())
                    .description(format!(
                        "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                        member.user.tag(),
                        ActionTypes::kick,
                        reason_default
                    ))
                    .timestamp(Timestamp::now())
            })
            .ephemeral(true)
        })
        .await;

    Ok(())
}
