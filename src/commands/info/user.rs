use poise::serenity_prelude::{Member, Timestamp, User};

use crate::{ternary, utils::{get_guild_member, Valeriyya}, Context, Error};

/// Gets the information about a user.
#[poise::command(slash_command, category = "Information", default_member_permissions="SEND_MESSAGES")]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Gets the information about a user."] user: Option<Member>,
) -> Result<(), Error> {
    let member = &user.unwrap_or(get_guild_member(ctx).await?.unwrap());

    ctx.send(Valeriyya::reply_default().embed(
        Valeriyya::embed()
            .author(Valeriyya::reply_author(format!("{} ({})", member.user.tag(), member.user.id)).icon_url(member.face()))
            .description(format!(
                "User Created At: {}\nMember Joined At: {}",
                format_args!(
                    "{} {}",
                    time_format(member.user.created_at()),
                    is_bot(&member.user)
                ),
                time_format(member.joined_at.unwrap())
            ))
    )).await;

    Ok(())
}

fn time_format(time: Timestamp) -> String {
    format!("<t:{}:d>", time.unix_timestamp())
}

fn is_bot(user: &User) -> &str {
    ternary!(user.bot => {
        "(User is a bot)";
        "";
    })
}
