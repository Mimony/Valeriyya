use crate::{serenity, ternary, utils::get_guild_member, Context, Error};

/// Gets the information about a user.
#[poise::command(prefix_command, slash_command, category = "Information")]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Gets the information about a user."] user: Option<serenity::Member>,
) -> Result<(), Error> {
    // poise::builtins::register_application_commands(ctx, false).await?;

    let member = &user.unwrap_or(get_guild_member(ctx).await?.unwrap());

    ctx.send(|m| {
        m.embed(|e| {
            e.color(serenity::Color::from_rgb(82, 66, 100));
            e.timestamp(serenity::Timestamp::now());
            e.author(|a| {
                a.name(format!("{} ({})", member.user.tag(), member.user.id));
                a.icon_url(format!("{}", member.face()))
            });
            e.description(format!(
                "User Created At: {}\nMember Joined At: {}",
                format!(
                    "{} {}",
                    time_format(member.user.created_at()),
                    is_bot(&member.user)
                ),
                time_format(member.joined_at.unwrap())
            ))
        })
        .ephemeral(true)
    })
    .await?;

    Ok(())
}

fn time_format(time: serenity::Timestamp) -> String {
    format!("<t:{}:d>", time.unix_timestamp())
}

fn is_bot(user: &serenity::User) -> &str {
    ternary!(user.bot => {
        "(User is a bot)";
        "";
    })
}
