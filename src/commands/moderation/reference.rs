use crate::{
    serenity,
    utils::{get_guild_db, update_case, CaseUpdateAction, CaseUpdateValue},
    Context, Error,
};

#[poise::command(prefix_command, slash_command, category = "Moderation", default_member_permissions="MANAGE_GUILD")]
pub async fn reference(
    ctx: Context<'_>,
    #[description = "The case to assign a reference."] case: u32,
    #[description = "The reference case"] reference: u32,
) -> Result<(), Error> {
    let database = &ctx.data().database;
    let guild_id = ctx.guild_id().unwrap().0;
    let db = get_guild_db(database, guild_id).await;

    let case_1 = db.cases.iter().find(|c| c.id == case);
    let case_2 = db.cases.iter().find(|c| c.id == reference);

    if case_1.is_none() & case_2.is_none() {
        ctx.send(|m| {
            m.content("Cases with this ids don't exist")
            .ephemeral(true)
        }).await;
        return Ok(())
    } else if case_1.is_none() {
        ctx.send(|m| {
            m.content(format!("Case with the id: {} doesn't exist", case))
            .ephemeral(true)
        }).await;
        return Ok(())
    } else if case_2.is_none() {
        ctx.send(|m| {
            m.content(format!("Case with the id: {} doesn't exist", reference))
            .ephemeral(true)
        }).await;
        return Ok(())
    }
    
    update_case(database, guild_id, case, CaseUpdateAction::reference, CaseUpdateValue {
        reason: None,
        reference: Some(reference)
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