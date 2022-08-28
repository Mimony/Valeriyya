use std::num::NonZeroU64;

use poise::{
    serenity_prelude::{ChannelId, CreateMessage, Timestamp},
    CreateReply,
};

use crate::{
    serenity,
    utils::{create_case, get_guild_db, member_managable, ActionTypes, Case, valeriyya_embed},
    Context, Error,
};

/// Kicks a member from the guild.
#[poise::command(
    slash_command,
    category = "Moderation",
    default_member_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member to kick"] member: serenity::Member,
    #[description = "The reason for this kick."]
    #[rest]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    let db = get_guild_db(database, guild_id).await;

    let reason_default = reason.unwrap_or_else(|| format!("Use /reason {} <...reason> to seat a reason for this case.", db.cases_number + 1));

    if !member_managable(ctx, &member).await {
        ctx.send(
            CreateReply::default()
                .content("The member can't be managed so you can't kick them!")
                .ephemeral(true),
        )
        .await;
        return Ok(());
    }
    member
        .kick_with_reason(ctx.discord(), &reason_default)
        .await;
    let icon_url = ctx
        .guild()
        .unwrap()
        .icon_url()
        .unwrap_or_else(|| String::from(""));

    let message = if db.channels.logs.is_some() {
        let sent_msg = ChannelId(db.channels.logs.unwrap().parse::<NonZeroU64>().unwrap())
            .send_message(
                ctx.discord(),
                CreateMessage::default().add_embed(
                    valeriyya_embed()
                    .author(
                        serenity::CreateEmbedAuthor::new(format!("{} ({})", ctx.author().tag(), ctx.author().id))
                            .icon_url(ctx.author().face()),
                    )
                        .thumbnail(&icon_url)
                        .description(format!(
                            "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                            member.user.tag(),
                            ActionTypes::kick,
                            reason_default
                        ))
                        .footer(serenity::CreateEmbedFooter::new(format!("Case {}", db.cases_number + 1)))
                ),
            )
            .await?;
        Some(sent_msg.id.to_string())
    } else {
        None
    };

    create_case(
        database,
        guild_id,
        Case {
            id: db.cases_number + 1,
            action: ActionTypes::kick,
            guild_id: guild_id.to_string(),
            staff_id: ctx.author().id.to_string(),
            target_id: member.user.id.to_string(),
            date: Timestamp::unix_timestamp(&Timestamp::now()),
            reason: reason_default.to_string(),
            message,
            expiration: None,
            reference: None,
        },
    )
    .await;

    ctx.say(format!("{} has been kicked by {}!", member, ctx.author())).await;

    Ok(())
}
