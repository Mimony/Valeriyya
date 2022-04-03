use poise::serenity_prelude::Timestamp;
use crate::{member_managable, serenity, string_to_sec, Context, Error};

#[poise::command(slash_command, category = "Moderation")]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "The member to mute"] mut member: serenity::Member,
    #[description = "The time the member to be muted for. (Max 28 days)."] time: String,
    #[description = "The reason for this mute."] reason: Option<String>,
) -> Result<(), Error> {
    let reason_default = reason.unwrap_or_else(|| String::from("default reason"));
    let timestamp = Timestamp::from_unix_timestamp(
        Timestamp::unix_timestamp(&Timestamp::now()) + string_to_sec(&time
    )).ok();

    if member_managable(ctx, &member).await {
        if member.communication_disabled_until.is_none() {
            ctx.send(|m| {
                m.content("This member is already muted");
                m.ephemeral(true)
            }).await;
        }


        member
            .disable_communication_until_datetime(&ctx.discord().http, timestamp.unwrap())
            .await;
        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100));
                e.author(|a| {
                    a.name(format!("{} ({})", member.user.tag(), member.user.id));
                    a.icon_url(ctx.author().face())
                });
                e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
                e.description(format!(
                    "Member: `{}`\nAction: `{}`\nReason: {}\nExpiration: {}",
                    member.user.tag(),
                    "mute",
                    reason_default,
                    time_format(time)
                ));
                e.timestamp(Timestamp::now())
            });
            s.ephemeral(true)
        })
        .await;
    };
    Ok(())
}

fn time_format(time: String) -> String {
    format!(
        "<t:{}:R>",
        Timestamp::unix_timestamp(&Timestamp::now()) + string_to_sec(time)
    )
}