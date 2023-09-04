use poise::serenity_prelude::{ChannelId, Timestamp, Member};

use crate::{
    structs::{ActionTypes, Case},
    utils::{member_managable,Valeriyya},
    Context, Error,
};

#[doc = "Kicks a member from the guild."]
#[poise::command(
    slash_command,
    category = "Moderation",
    default_member_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member to kick"] member: Member,
    #[description = "The reason for this kick."]
    #[rest]
    reason: Option<String>,
) -> Result<(), Error> {
    let database = &ctx.data().database();
    let guild_id = ctx.guild_id().unwrap();

    let mut guild_db = Valeriyya::get_database(database, guild_id.to_string()).await;
    let case_number = guild_db.cases_number + 1;
    let reason_default = reason.unwrap_or_else(|| format!("Use /reason {} <...reason> to set a reason for this case.", case_number));

    if !member_managable(ctx, &member).await {
        ctx.send(Valeriyya::reply("The member can't be managed so you can't kick them!").ephemeral(true)).await?;
        return Ok(());
    }
    member
        .kick_with_reason(ctx.discord(), &reason_default)
        .await?;
    let icon_url = ctx
        .guild()
        .unwrap()
        .icon_url()
        .unwrap_or_else(|| String::from(""));

    let message = if guild_db.channels.logs.as_ref().is_some() {
        let sent_msg = ChannelId(guild_db.channels.logs.as_ref().unwrap().parse::<std::num::NonZeroU64>().unwrap())
            .send_message(ctx.discord(), Valeriyya::msg_reply().add_embed(
                Valeriyya::embed()
                    .author(Valeriyya::reply_author(format!(
                        "{} ({})",
                        ctx.author().tag(),
                        ctx.author().id
                    )).icon_url(ctx.author().face()))
                    .thumbnail(&icon_url)
                    .description(format!(
                        "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                        member.user.tag(),
                        ActionTypes::Kick,
                        reason_default
                    ))
                    .footer(Valeriyya::reply_footer(format!("Case {}", case_number)))
            )).await.expect("Guild log channel doesn't exist");

        Some(sent_msg.id.to_string())
    } else {
        None
    };

    guild_db = guild_db.add_cases(Case {
        id: case_number,
        action: ActionTypes::Kick,
        guild_id: guild_id.to_string(),
        staff_id: ctx.author().id.to_string(),
        target_id: member.user.id.to_string(),
        date: Timestamp::unix_timestamp(&Timestamp::now()),
        reason: reason_default.to_string(),
        message,
        expiration: None,
        reference: None,
    });

    ctx.say(format!("{} has been kicked by {}!", member, ctx.author())).await?;

    guild_db.execute(database).await;
    Ok(())
}
