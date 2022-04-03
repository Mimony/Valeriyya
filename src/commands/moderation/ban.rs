use poise::serenity_prelude::{UserId, Timestamp};
use crate::{serenity, Context, Error, member_managable};

#[poise::command(slash_command, category = "Moderation")]
pub async fn ban(ctx: Context<'_>, 
    #[description="The member to ban"] member: Option<serenity::Member>,
    #[description="The member to ban (Use this to provide an id instead of mention)"] member_id: Option<u64>,
    #[description="The reason for this ban."] reason: Option<String>,
) -> Result<(), Error> {

    let reason_default = reason.unwrap_or_else(|| String::from("Default reason"));

    if let Some(m) = &member {

        if member_managable(ctx, m).await {      
            if ctx.guild().unwrap().bans(&ctx.discord().http).await?.iter().any(|ban| ban.user.id == m.user.id) {
                ctx.send(|int| {
                    int.content("This member is already banned from this guild.");
                    int.ephemeral(true)
                }).await;
            }      
            m.ban_with_reason(ctx.discord(), 7, &reason_default).await?;
            ctx.send(|s| {
                s.embed(|e| {
                    e.color(serenity::Color::from_rgb(82, 66, 100));
                    e.author(|a| {
                        a.name(format!("{} ({})", m.user.tag(), m.user.id));
                        a.icon_url(ctx.author().face())
                    });
                    e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
                    e.description(format!("Member: `{}`\nAction: `{}`\nReason: {}", m.user.tag(), "ban", reason_default));
                    e.timestamp(Timestamp::now())
                });
                s.ephemeral(true)
            }).await;
        };
    }  
    if let Some(m_id) = &member_id {
        if ctx.guild().unwrap().bans(&ctx.discord().http).await?.iter().any(|ban| ban.user.id == *m_id) {
            ctx.send(|int| {
                int.content("This member is already banned from this guild.");
                int.ephemeral(true)
            }).await;
        }
        let user_id = UserId(*m_id);
        ctx.guild().unwrap().ban_with_reason(ctx.discord(), user_id, 7, &reason_default).await?;
        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100));
                e.author(|a| {
                    a.name(format!("{} ({})", ctx.author().tag(), ctx.author().id));
                    a.icon_url(ctx.author().face())
                });
                e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
                e.description(format!("Member: `{}`\nAction: `{}`\nReason: {}", m_id, "mute", reason_default));
                e.timestamp(Timestamp::now())
            });
            s.ephemeral(true)
         }).await;
    }

    Ok(())
}