use crate::{
    serenity,
    utils::{member_managable, string_to_sec, ActionTypes, Case, valeriyya_embed, GuildDb},
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

    let database = &ctx.data().database();
    let guild_id = ctx.guild_id().unwrap();

    let mut guild_db = GuildDb::new(database, guild_id.to_string()).await;
    let case_number = guild_db.cases_number + 1;
    let reason_default = reason.unwrap_or_else(|| format!("Use /reason {} <...reason> to seat a reason for this case.", case_number));

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
    let message = if guild_db.channels.logs.is_some() {
        let sent_msg = serenity::ChannelId(
            guild_db.channels
                .logs
                .as_ref()
                .unwrap()
                .parse::<std::num::NonZeroU64>()
                .unwrap(),
        )
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
                        "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration: {}",
                        member.user.tag(),
                        ActionTypes::Mute,
                        reason_default,
                        time_format(time)
                    ))
                    .footer(
                        serenity::CreateEmbedFooter::new(format!("Case {}", case_number)),
                    ),
            ),
        )
        .await?;
        Some(sent_msg.id.to_string())
    } else {
        None
    };

    guild_db = guild_db.add_cases(Case {
        id: case_number,
        action: ActionTypes::Mute,
        guild_id: guild_id.0.to_string(),
        staff_id: ctx.author().id.to_string(),
        target_id: member.user.id.to_string(),
        date: Timestamp::unix_timestamp(&Timestamp::now()),
        reason: reason_default.to_string(),
        expiration: Some(timestamp.unwrap().unix_timestamp()),
        message,
        reference: None,
    });

    ctx.say(format!("{} has been muted by {}!", member, ctx.author())).await;

    guild_db.execute(database).await;
    Ok(())
}

fn time_format(time: String) -> String {
    format!(
        "<t:{}:R>",
        Timestamp::unix_timestamp(&Timestamp::now()) + string_to_sec(time)
    )
}
