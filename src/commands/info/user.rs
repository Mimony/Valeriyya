use poise::{CreateReply, serenity_prelude::{CreateEmbed, CreateEmbedAuthor}};

use crate::{serenity, ternary, utils::get_guild_member, Context, Error};

/// Gets the information about a user.
#[poise::command(prefix_command, slash_command, category = "Information", default_member_permissions="SEND_MESSAGES")]
pub async fn user(
    ctx: Context<'_>,
    #[description = "Gets the information about a user."] user: Option<serenity::Member>,
) -> Result<(), Error> {
    let member = &user.unwrap_or(get_guild_member(ctx).await?.unwrap());

    ctx.send(CreateReply::default()
        .embed(CreateEmbed::default()
            .color(serenity::Color::from_rgb(82, 66, 100))
            .timestamp(serenity::Timestamp::now())
            .author(CreateEmbedAuthor::default()
                .name(format!("{} ({})", member.user.tag(), member.user.id))
                .icon_url(member.face())
            )
            .description(format!(
                "User Created At: {}\nMember Joined At: {}",
                format!(
                    "{} {}",
                    time_format(member.user.created_at()),
                    is_bot(&member.user)
                ),
                time_format(member.joined_at.unwrap())
            ))
        )
        .ephemeral(true)
    )
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
