use crate::{serenity, Context, Error, member_managable};

#[poise::command(slash_command, category = "Moderation")]
pub async fn kick(ctx: Context<'_>, 
    #[description="The member to kick"] member: serenity::Member,
    #[description="The reason for this kick."] _reason_option: Option<String>,
) -> Result<(), Error> {

    // let reason = reason_option.unwrap_or_else(|| String::from("Default reason"));


        if member_managable(ctx, &member).await {            
            member.kick(ctx.discord()).await;
            ctx.send(|s| {
                s.embed(|e| {
                    e.author(|a| {
                        a.name(format!("{} ({})", member.user.tag(), member.user.id));
                        a.icon_url(ctx.author().avatar_url().unwrap_or_else(|| String::from("")))
                    });
                    e.thumbnail(ctx.guild().unwrap().icon.unwrap_or_else(|| String::from("")));
                    e.description(format!("{} has been kicked from {}", member.user.tag(), ctx.guild().unwrap().name))
                });
                s.ephemeral(true)
            }).await;
        };

    Ok(())
}