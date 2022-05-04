use crate::{
    serenity,
    utils::{get_guild_db, update_case, CaseUpdateAction, CaseUpdateValue},
    Context, Error,
};

#[poise::command(prefix_command, slash_command, category = "Moderation")]
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
        ctx.send(|m| {
            m.content(format!("Case with the id: {} doesn't exist", case))
            .ephemeral(true)
        }).await;
        return Ok(())
    } 
    
    update_case(database, guild_id, case, CaseUpdateAction::reason, CaseUpdateValue {
        reason: Some(reason),
        reference: None
    });

    ctx.send(|m| {
        m.content(format!("Updated case with the id: {case}"))
        .ephemeral(true)
    }).await;

    if db.channels.logs.is_some() {
        let channel = serenity::ChannelId(db.channels.logs.unwrap().parse::<u64>().unwrap());
        channel.say(ctx.discord(), "smth").await;
    }

    Ok(())
}