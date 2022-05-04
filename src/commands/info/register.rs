use crate::{Context, Error};

#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;

    Ok(())
}