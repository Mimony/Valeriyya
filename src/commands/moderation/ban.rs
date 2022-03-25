use poise::serenity_prelude::{UserId};

use crate::{serenity, Context, Error, member_managable};


#[poise::command(slash_command, category = "Moderation")]
pub async fn ban(ctx: Context<'_>, 
    #[description="The member to ban"] member: Option<serenity::Member>,
    #[description="The member to ban (Use this to provide an id instead of mention)"] member_id: Option<u64>,
    #[description="The reason for this ban."] reason_option: Option<String>,
) -> Result<(), Error> {

    let reason = reason_option.unwrap_or_else(|| String::from("Default reason"));

    if let Some(m) = &member {

        if member_managable(ctx, m).await {            
            m.ban_with_reason(ctx.discord(), 7, &reason).await;
            ctx.send(|s| {
                s.embed(|e| {
                    e.author(|a| {
                        a.name(format!("{} ({})", m.user.tag(), m.user.id));
                        a.icon_url(ctx.author().avatar_url().unwrap_or_else(|| String::from("")))
                    });
                    e.thumbnail(ctx.guild().unwrap().icon.unwrap_or_else(|| String::from("")));
                    e.description(format!("{} has been banned from {}", m.user.tag(), ctx.guild().unwrap().name))
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
        ctx.guild().unwrap().ban_with_reason(ctx.discord(), user_id, 7, &reason).await;
        ctx.send(|s| {
            s.embed(|e| {
                e.author(|a| {
                    a.name(format!("{} ({})", "Unknown Tag", m_id));
                    a.icon_url(ctx.author().avatar_url().unwrap_or_else(|| String::from("")))
                });
                e.thumbnail(ctx.guild().unwrap().icon.unwrap_or_else(|| String::from("")));
                e.description(format!("{} has been banned from {}", m_id, ctx.guild().unwrap().name))
            });
            s.ephemeral(true)
         }).await;
    }

    Ok(())
}