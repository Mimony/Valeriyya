use crate::{
    serenity,
    utils::{create_case, get_guild_db, member_managable, ActionTypes, Case},
    Context, Error,
};
use poise::serenity_prelude::{Timestamp, UserId};

/// Bans a member from the guild.
#[poise::command(prefix_command, slash_command, category = "Moderation", default_member_permissions="BAN_MEMBERS")]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The member to ban"] member: Option<serenity::Member>,
    #[description = "The member to ban (Use this to provide an id instead of mention)"]
    member_id: Option<String>,
    #[description = "The reason for this ban."] #[rest] reason: Option<String>,
) -> Result<(), Error> {
    let reason_default = reason.unwrap_or_else(|| String::from("Default reason"));
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap().0;

    let db = get_guild_db(database, guild_id).await;

    if let Some(m) = &member {
        if !member_managable(ctx, m).await {
            ctx.send(|m| {
                m.content("The member can't be managed so you can't ban them!")
                    .ephemeral(true)
            })
            .await;
            return Ok(());
        }
        if ctx
            .guild()
            .unwrap()
            .bans(&ctx.discord().http)
            .await?
            .iter()
            .any(|ban| ban.user.id == m.user.id)
        {
            ctx.send(|int| {
                int.content("This member is already banned from this guild.")
                    .ephemeral(true)
            })
            .await;
        }
        m.ban_with_reason(ctx.discord(), 7, &reason_default).await?;
        create_case(
            database,
            ctx.guild_id().unwrap().0,
            Case {
                id: db.cases_number + 1,
                action: ActionTypes::ban,
                guild_id: guild_id.to_string(),
                staff_id: ctx.author().id.to_string(),
                target_id: m.user.id.to_string(),
                date: Timestamp::unix_timestamp(&Timestamp::now()),
                reason: reason_default.to_string(),
                expiration: None,
                reference: None, 
            },
        )
        .await;
        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100))
                    .author(|a| {
                        a.name(format!("{} ({})", m.user.tag(), m.user.id));
                        a.icon_url(ctx.author().face())
                    })
                    .thumbnail(ctx.guild().unwrap().icon_url().unwrap())
                    .description(format!(
                        "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                        m.user.tag(),
                        ActionTypes::ban,
                        reason_default
                    ))
                    .timestamp(Timestamp::now())
            })
            .ephemeral(true)
        })
        .await;
    }
    if let Some(m_id) = &member_id {
        let user_id = UserId(m_id.parse().unwrap());
        if ctx
            .guild()
            .unwrap()
            .bans(&ctx.discord().http)
            .await?
            .iter()
            .any(|ban| ban.user.id == user_id)
        {
            ctx.send(|int| {
                int.content("This member is already banned from this guild.");
                int.ephemeral(true)
            })
            .await;
        }
        ctx.guild()
            .unwrap()
            .ban_with_reason(ctx.discord(), user_id, 7, &reason_default)
            .await?;

        create_case(
            database,
            ctx.guild_id().unwrap().0,
            Case {
                id: db.cases_number + 1,
                action: ActionTypes::ban,
                guild_id: guild_id.to_string(),
                staff_id: ctx.author().id.to_string(),
                target_id: user_id.0.to_string(),
                date: Timestamp::unix_timestamp(&Timestamp::now()),
                reason: reason_default.to_string(),
                expiration: None,  
                reference: None, 
            },
        )
        .await;

        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100));
                e.author(|a| {
                    a.name(format!("{} ({})", ctx.author().tag(), ctx.author().id));
                    a.icon_url(ctx.author().face())
                });
                e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
                e.description(format!(
                    "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                    m_id,
                    ActionTypes::ban,
                    reason_default
                ));
                e.timestamp(Timestamp::now())
            });
            s.ephemeral(true)
        })
        .await;
    }

    Ok(())
}
