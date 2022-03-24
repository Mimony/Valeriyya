use crate::{serenity, Context, Error, member_managable};

use self::serenity::UserId;

#[poise::command(slash_command, category = "Moderation")]
pub async fn ban(ctx: Context<'_>, 
    #[description="The member to ban"] member: Option<serenity::Member>,
    #[description="The member to ban (Use this to provide an id instead of mention)"] member_id: Option<u64>,
    #[description="The reason for this ban."] reason_option: Option<String>,
) -> Result<(), Error> {

    let reason = reason_option.unwrap_or_else(|| String::from("Default reason"));

    if let Some(m) = &member {
        println!("{}", m.roles.len());
        if member_managable(ctx, m).await {
            println!("{}", m.roles.len());
            // m.ban_with_reason(ctx.discord(), 7, &reason).await;
            // ctx.send(|s| {
            //    s.content(format!("{} has been banned", m.user.name));
            //    s.ephemeral(true)
            // });
        };
    }  
    if let Some(m_id) = &member_id {
        println!("yes3");
        let user_id = UserId(*m_id);
        ctx.guild().unwrap().ban_with_reason(ctx.discord(), user_id, 7, &reason);
        ctx.send(|s| {
            s.content(format!("User with the id {} has been banned", user_id));
            s.ephemeral(true)
         });
    }

    Ok(())
}