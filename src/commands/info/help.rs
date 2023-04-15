use crate::{Context, Error};

#[poise::command(prefix_command, guild_only)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        Default::default(),
    )
    .await?;

    Ok(())
}
