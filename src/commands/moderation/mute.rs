use poise::serenity_prelude::Timestamp;

use crate::{serenity, Context, Error, member_managable, string_to_sec};

#[poise::command(slash_command, category = "Moderation")]
pub async fn mute(ctx: Context<'_>, 
    #[description="The member to mute"] mut member: serenity::Member,
    #[description="The time the member to be muted for. (Max 28 days)."] time: String,
    #[description="The reason for this mute."] reason: Option<String>,
) -> Result<(), Error> {

    let reason_default = reason.unwrap_or_else(|| String::from("default reason"));
    let timestamp = Timestamp::from_unix_timestamp(Timestamp::unix_timestamp(&Timestamp::now()) + string_to_sec(&time)).ok();

        if member_managable(ctx, &member).await {   
            member.disable_communication_until_datetime(&ctx.discord().http, timestamp.unwrap()).await;
            ctx.send(|s| {
                s.embed(|e| {
                    // e.author(|a| {
                    //     a.name(format!("{} ({})", member.user.tag(), member.user.id));
                    //     a.icon_url(ctx.author().face()))
                    // });
                    e.thumbnail(ctx.guild().unwrap().icon.unwrap_or_else(|| String::from("")));
                    e.description(format!("{} has been muteed from {} with the reason: {}", member.user.tag(), ctx.guild().unwrap().name, reason_default))
                });
                s.ephemeral(true)
            }).await;
        };
    Ok(())
}