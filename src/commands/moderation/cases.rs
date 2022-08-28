use std::num::NonZeroU64;

use poise::{
    serenity_prelude::{CreateEmbedAuthor, CreateEmbedFooter, Timestamp, UserId},
    CreateReply,
};

use crate::{
    serenity,
    utils::{get_guild_db, get_guild_member, update_guild_db, ActionTypes, valeriyya_embed},
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
    let database = &ctx.data().database;

    let guild_id = ctx.guild_id().unwrap().0;

    let mut db = get_guild_db(database, guild_id).await;
    let staff = get_guild_member(ctx).await?.unwrap();

    if let OptionChoices::Show = option {
        let case = db.cases.iter().find(|c| c.id == id);

        if case.is_none() {
            ctx.send(
                CreateReply::default()
                    .content(format!("Can't find a case with the id: {}", id))
                    .ephemeral(true),
            )
            .await;
            return Ok(());
        }

        let case = case.unwrap();
        let target_user = UserId(case.target_id.parse::<NonZeroU64>().unwrap()).to_user(ctx.discord()).await?.tag();

        let mut case_embed = valeriyya_embed()
            .author(
                CreateEmbedAuthor::new(format!("{} ({})", staff.user.tag(), staff.user.id))
                    .icon_url(staff.user.face()),
            )
            .thumbnail(ctx.guild().unwrap().icon_url().unwrap())
            .timestamp(Timestamp::from(&Timestamp::from_unix_timestamp(case.date).unwrap()))
            .footer(CreateEmbedFooter::new(format!("Case {}", case.id)));
        if ActionTypes::mute == case.action && case.reference.is_some() {
            case_embed = case_embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration:<t:{}:R>\nReference: `{}`",
                target_user, case.action, case.reason, case.expiration.unwrap(), case.reference.unwrap()
            ));
        } else if ActionTypes::mute == case.action {
            case_embed = case_embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration:<t:{}:R>",
                target_user, case.action, case.reason, case.expiration.unwrap()
            ));
        } else if case.reference.is_some() {
            case_embed = case_embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nReference: `{}`",
                target_user, case.action, case.reason, case.reference.unwrap()
            ));
        } else {
            case_embed = case_embed
            .description(format!(
                "Member: `{}`\nAction: `{:?}`\nReason: {}\n",
                target_user, case.action, case.reason
            ));
        }
        
        ctx.send(CreateReply::default().embed(case_embed).ephemeral(true))
            .await;
    } else if let OptionChoices::Delete = option {
        let case = db.cases.iter().find(|c| c.id == id);

        if case.is_none() {
            ctx.send(
                CreateReply::default()
                    .content(format!("Can't find a case with the id: {}", id))
                    .ephemeral(true),
            )
            .await;
            return Ok(());
        }

        let case = case.unwrap();

        let index = db
            .cases
            .iter()
            .position(|indexc| indexc.id == case.id)
            .unwrap();
        db.cases.remove(index);

        update_guild_db(database, guild_id, &db).await;

        ctx.send(
            CreateReply::default()
                .embed(
                    valeriyya_embed()
                        .author(
                            CreateEmbedAuthor::new(format!("{} ({})", staff.user.tag(), staff.user.id))
                                .icon_url(staff.user.face()),
                        )
                        .description(format!("Case with the id: {} has been deleted.", id))
                        .timestamp(serenity::Timestamp::now()),
                )
                .ephemeral(true),
        )
        .await;
    }

    Ok(())
}
