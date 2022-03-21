use crate::{serenity, Context, Error};

#[poise::command(
    slash_command,
    category = "Information"
)]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Gets the information about a user."] user: Option<serenity::User>,
) -> Result<(), Error> {
        ctx.send(|f| {
            f
            .content(format!("{}", match user {
                Some(u) => u.name,
                None => "Please provide a user (test)".to_string(),
            }))
            .ephemeral(true)
        }).await?;
    
Ok(())
}