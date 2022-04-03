use poise::serenity_prelude::Timestamp;

use crate::{serenity, Context, Error, member_managable};

#[poise::command(slash_command, category = "Moderation")]
pub async fn kick(ctx: Context<'_>, 
    #[description="The member to kick"] member: serenity::Member,
    #[description="The reason for this kick."] reason: Option<String>,
) -> Result<(), Error> {

    let reason_default = reason.unwrap_or_else(|| String::from("Default reason"));


        if member_managable(ctx, &member).await {            
            member.kick(ctx.discord()).await;
            ctx.send(|s| {
                s.embed(|e| {
                    e.color(serenity::Color::from_rgb(82, 66, 100));
                    e.author(|a| {
                        a.name(format!("{} ({})", member.user.tag(), member.user.id));
                        a.icon_url(ctx.author().face())
                    });
                    e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
                    e.description(format!("Member: `{}`\nAction: `{}`\nReason: {}", member.user.tag(), "kick", reason_default));
                    e.timestamp(Timestamp::now())
                });
                s.ephemeral(true)
            }).await;
        };

    Ok(())
}