use poise::{
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor, Timestamp},
    CreateReply,
};

use crate::{
    serenity,
    utils::{create_case, get_guild_db, member_managable, ActionTypes, Case},
    Context, Error,
};

/// Kicks a member from the guild.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    default_member_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member to kick"] member: serenity::Member,
    #[description = "The reason for this kick."]
    #[rest]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap();

    let db = get_guild_db(database, guild_id).await;

    let reason_default = reason.unwrap_or_else(|| String::from("Default reason"));

    if !member_managable(ctx, &member).await {
        ctx.send(
            CreateReply::default()
                .content("The member can't be managed so you can't kick them!")
                .ephemeral(true),
        )
        .await;
        return Ok(());
    }
    member
        .kick_with_reason(ctx.discord(), &reason_default)
        .await;
    create_case(
        database,
        guild_id,
        Case {
            id: db.cases_number + 1,
            action: ActionTypes::kick,
            guild_id: guild_id.to_string(),
            staff_id: ctx.author().id.to_string(),
            target_id: member.user.id.to_string(),
            date: Timestamp::unix_timestamp(&Timestamp::now()),
            reason: reason_default.to_string(),
            expiration: None,
            reference: None,
        },
    )
    .await;
        let icon_url = ctx
            .guild()
            .unwrap()
            .icon_url()
            .unwrap_or_else(|| String::from(""));
        ctx.send(
            CreateReply::default()
                .embed(
                    CreateEmbed::default()
                        .color(serenity::Color::from_rgb(82, 66, 100))
                        .author(
                            CreateEmbedAuthor::default()
                                .name(format!("{} ({})", member.user.tag(), member.user.id))
                                .icon_url(ctx.author().face()),
                        )
                        .thumbnail(icon_url)
                        .description(format!(
                            "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                            member.user.tag(),
                            ActionTypes::kick,
                            reason_default
                        ))
                        .timestamp(Timestamp::now()),
                )
                .ephemeral(true),
        )
        .await;
    

    Ok(())
}
