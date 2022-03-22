use crate::{serenity, Context, Error};

#[poise::command(slash_command, category = "Information")]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Gets the information about a user."] user: Option<serenity::User>,
) -> Result<(), Error> {
    ctx.send(|f| {
        f
            .embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100));
                e.timestamp(serenity::Timestamp::now());
                e.author(|a| {
                    a.name(format!(
                        "{}",
                        match &user {
                            Some(u) => format!("{} ({})", u.tag(), u.id),
                            None => format!("{} ({})", ctx.author().tag(), ctx.author().id),
                        }
                    ));
                    a.icon_url(format!(
                        "{}",
                        match &user {
                            Some(u) => u.avatar_url().unwrap(),
                            None => ctx.author().avatar_url().unwrap(),
                        }
                    ))
                });
                e.description(format!(
                    "User created at: {}\nMember Joined At:",
                    match &user {
                        Some(u) => format!("{} {}", time_format(u.created_at()), is_bot(u)),
                        None => format!(
                            "{} {}",
                            time_format(ctx.author().created_at()),
                            is_bot(ctx.author())
                        ),
                    }
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

fn is_bot(user: &serenity::User) -> &'static str {
    if user.bot {
        return "(User is a bot)";
    }
    ""
}