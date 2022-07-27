use std::num::NonZeroU64;

use poise::CreateReply;

use crate::{
    serenity,
    utils::{get_guild_db, update_case, CaseUpdateAction, CaseUpdateValue},
    Context, Error,
};

#[poise::command(prefix_command, slash_command, category = "Moderation", default_member_permissions="MANAGE_GUILD")]
pub async fn reason(
    ctx: Context<'_>,
    #[description = "The case to assign a reason."] case: u32,
    #[description = "The reasoning for the case."] reason: String,
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
        reason: Some(reason),
        reference: None
    });

    ctx.send(CreateReply::default().content(format!("Updated case with the id: {case}"))
        .ephemeral(true)
    ).await;

    if db.channels.logs.is_some() {
        let channel = serenity::ChannelId(db.channels.logs.unwrap().parse::<NonZeroU64>().unwrap());
        channel.say(ctx.discord(), format!("Temporary msg")).await;
    }

    Ok(())
}