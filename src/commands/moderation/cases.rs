use poise::serenity_prelude::{Timestamp, UserId, CreateEmbed, Member};

use crate::{
    utils::{get_guild_member, ActionTypes, Valeriyya, GuildDb, Case},
    Context, Error,
};

#[derive(poise::ChoiceParameter)]
pub enum OptionChoices {
    #[name = "show"]
    Show,
    #[name = "delete"]
    Delete,
}

/// Shows or deletes a case from guilds actions.
#[poise::command(
    slash_command,
    category = "Moderation",
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn cases(
    ctx: Context<'_>,
    #[description = "What to do with the case."] option: OptionChoices,
    #[description = "The id of the case."] id: u32,
) -> Result<(), Error> {
    let database = &ctx.data().database();

    let guild_id = ctx.guild_id().unwrap().0;

    let mut db = GuildDb::new(database, guild_id.to_string()).await;
    let staff = get_guild_member(ctx).await?.unwrap();

    if let OptionChoices::Show = option {
        let case = db.cases.iter().find(|c| c.id == id);

        if case.is_none() {
            ctx.send(Valeriyya::reply(format!("Can't find a case with the id: {}", id)).ephemeral(true))
            .await;
            return Ok(());
        }
        
        let case = case.unwrap();
        let target_user = UserId(case.target_id.parse::<std::num::NonZeroU64>().unwrap()).to_user(ctx.discord()).await?.tag();

        ctx.send(Valeriyya::reply_default().embed(create_embed(ctx, staff, case, target_user))).await;
    } else if let OptionChoices::Delete = option {
        let case = db.cases.iter().find(|c| c.id == id);

        if case.is_none() {
            ctx.send(Valeriyya::reply(format!("Can't find a case with the id: {}", id)).ephemeral(true)).await;
            return Ok(());
        }

        let case = case.unwrap();

        let index = db
            .cases
            .iter()
            .position(|indexc| indexc.id == case.id)
            .unwrap();

        db = db.delete_cases(index);

        ctx.send(Valeriyya::reply_default().embed(
            Valeriyya::embed()
                .author(Valeriyya::reply_author(format!("{} ({})", staff.user.tag(), staff.user.id)).icon_url(staff.user.face()))
        ))
        .await;
    }
    db.execute(database).await;
    Ok(())
}

fn create_embed(ctx: Context<'_>, staff: Member, case: &Case, target_user: String) -> CreateEmbed {
    let mut embed = Valeriyya::embed()
            .author(Valeriyya::reply_author(format!("{} ({})", staff.user.tag(), staff.user.id)).icon_url(staff.user.face()))
            .thumbnail(ctx.guild().unwrap().icon_url().unwrap())
            .timestamp(Timestamp::from(&Timestamp::from_unix_timestamp(case.date).unwrap()))
            .footer(Valeriyya::reply_footer(format!("Case {}", case.id)));
        if ActionTypes::Mute == case.action && case.reference.is_some() {
            embed = embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration:<t:{}:R>\nReference: `{}`",
                target_user, case.action, case.reason, case.expiration.unwrap(), case.reference.unwrap()
            ));
        } else if ActionTypes::Mute == case.action {
            embed = embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration:<t:{}:R>",
                target_user, case.action, case.reason, case.expiration.unwrap()
            ));
        } else if case.reference.is_some() {
            embed = embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nReference: `{}`",
                target_user, case.action, case.reason, case.reference.unwrap()
            ));
        } else {
            embed = embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: {}\n",
                target_user, case.action, case.reason
            ));
        }
        embed
}