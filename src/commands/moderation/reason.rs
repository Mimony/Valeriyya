use std::num::NonZeroU64;

use poise::{CreateReply, serenity_prelude::{MessageId, EditMessage, UserId, Timestamp, CreateEmbedFooter}};

use crate::{
    serenity,
    utils::{get_guild_db, update_case, CaseUpdateAction, CaseUpdateValue, ActionTypes, valeriyya_embed},
    Context, Error,
};

#[poise::command(prefix_command, slash_command, category = "Moderation", default_member_permissions="MANAGE_GUILD")]
pub async fn reason(
    ctx: Context<'_>,
    #[description = "The case to assign a reason."] case: u32,
    #[description = "The reasoning for the case."] #[rest] reason: String,
) -> Result<(), Error> {
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap().0;
    let db = get_guild_db(database, guild_id).await;

    let case_find = db.cases.iter().find(|c| c.id == case);


     if case_find.is_none() {
        ctx.send(CreateReply::default().content(format!("Case with the id: {} doesn't exist", case))
            .ephemeral(true)
        ).await;
        return Ok(())
    } 
    
    update_case(database, guild_id, case, CaseUpdateAction::reason, CaseUpdateValue {
        reason: Some(reason.clone()),
        reference: None
    });

    ctx.send(CreateReply::default().content(format!("Updated case with the id: {case}"))
        .ephemeral(true)
    ).await;

    if db.channels.logs.is_some() {
        let channel = serenity::ChannelId(db.channels.logs.unwrap().parse::<NonZeroU64>().unwrap());
        if case_find.unwrap().message.is_some() {
            let case_found = case_find.unwrap();
            let mut log_channel_msg = channel.message(
                ctx.discord(), 
                MessageId(case_found.message.as_deref().unwrap().parse::<NonZeroU64>().unwrap())).await?;
            let staff_user_cache = UserId(case_found.staff_id.parse::<NonZeroU64>().unwrap()).to_user(ctx.discord()).await?.to_owned();
            let staff_user = (staff_user_cache.tag(), staff_user_cache.id, staff_user_cache.face());
            let target_user = UserId(case_found.target_id.parse::<NonZeroU64>().unwrap()).to_user(ctx.discord()).await?.tag();
            let mut embed = valeriyya_embed().timestamp(Timestamp::from(&Timestamp::from_unix_timestamp(case_found.date).unwrap()))
            .author(serenity::CreateEmbedAuthor::default().name(format!("{} ({})", staff_user.0, staff_user.1)).icon_url(staff_user.2))
            .footer(CreateEmbedFooter::default().text(format!("Case {}", case_found.id)));

            if case_found.action == ActionTypes::mute {
                embed = embed.description(format!(
                    "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration: {:?}",
                    target_user, case_found.action, reason, case_found.expiration.unwrap() 
                ));
            } else {
                embed = embed.description(format!(
                    "Member: `{}`\nAction: `{:?}`\nReason: `{}`",
                    target_user, case_found.action, reason
                ));
            };

            let edit = EditMessage::default().embed(embed);
            log_channel_msg.edit(ctx.discord(), edit).await;
        };
    }

    Ok(())
}