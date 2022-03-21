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
        .content(format!("{:?}", user))
        .ephemeral(true)
    }).await?;
    Ok(())
}