use crate::{
    serenity,
    utils::{create_case, get_guild_db, member_managable, string_to_sec, ActionTypes, Case, valeriyya_embed},
    Context, Error,
};
use poise::{
    serenity_prelude::{CreateMessage, Timestamp},
    CreateReply,
};

/// Mutes a member for a specified time.
#[poise::command(
    slash_command,
    category = "Moderation",
    default_member_permissions = "MODERATE_MEMBERS"
)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The member to mute"] mut member: serenity::Member,
    #[description = "The time the member to be muted for. (Max 28 days)."] time: String,
    #[description = "The reason for this mute."]
    #[rest]
    reason: Option<String>,
) -> Result<(), Error> {
    let string_time = string_to_sec(&time);

    if string_time < 60 {
        ctx.send(
            CreateReply::default()
                .content("You can't mute someone for under 60 seconds!")
                .ephemeral(true),
        )
        .await;
        return Ok(());
    }

    let timestamp =
        Timestamp::from_unix_timestamp(Timestamp::unix_timestamp(&Timestamp::now()) + string_time)
            .ok();

    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    let db = get_guild_db(database, guild_id.0).await;
    let reason_default = reason.unwrap_or_else(|| format!("Use /reason {} <...reason> to seat a reason for this case.", db.cases_number + 1));

    if !member_managable(ctx, &member).await {
        ctx.send(
            CreateReply::default()
                .content("The member can't be managed so you can't mute them!")
                .ephemeral(true),
        )
        .await;
        return Ok(());
    }

    if member.communication_disabled_until.is_some()
        && ((Timestamp::unix_timestamp(&Timestamp::now())
            - member
                .communication_disabled_until
                .unwrap()
                .unix_timestamp())
            < 0)
    {
        ctx.send(
            CreateReply::default()
                .content("This member is already muted")
                .ephemeral(true),
        )
        .await;
        return Ok(());
    };

    member
        .disable_communication_until_datetime(&ctx.discord().http, timestamp.unwrap())
        .await;
    let icon_url = ctx
        .guild()
        .unwrap()
        .icon_url()
        .unwrap_or_else(|| String::from(""));
    let message = if db.channels.logs.is_some() {
        let sent_msg = serenity::ChannelId(
            db.channels
                .logs
                .unwrap()
                .parse::<std::num::NonZeroU64>()
                .unwrap(),
        )
        .send_message(
            ctx.discord(),
            CreateMessage::default().add_embed(
                valeriyya_embed()
                    .author(
                        serenity::CreateEmbedAuthor::default()
                            .name(format!("{} ({})", ctx.author().tag(), ctx.author().id))
                            .icon_url(ctx.author().face()),
                    )
                    .thumbnail(&icon_url)
                    .description(format!(
                        "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration: {}",
                        member.user.tag(),
                        ActionTypes::mute,
                        reason_default,
                        time_format(time)
                    ))
                    .footer(
                        serenity::CreateEmbedFooter::default()
                            .text(format!("Case {}", db.cases_number + 1)),
                    ),
            ),
        )
        .await?;
        Some(sent_msg.id.to_string())
    } else {
        None
    };

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
            message,
            reference: None,
        },
    )
    .await;

    ctx.say(format!("{} has been muted by {}!", member, ctx.author())).await;

    Ok(())
}

fn time_format(time: String) -> String {
    format!(
        "<t:{}:R>",
        Timestamp::unix_timestamp(&Timestamp::now()) + string_to_sec(time)
    )
}
