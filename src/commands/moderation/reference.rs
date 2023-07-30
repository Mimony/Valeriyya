use std::num::NonZeroU64;

use poise::{
    serenity_prelude::{ChannelId, MessageId, Timestamp, UserId}
};

use crate::{
    structs::{ActionTypes, GuildDb, Case, CaseUpdateAction, CaseUpdateValue},
    utils::{update_case, Valeriyya},
    Context, Error,
};

#[doc = "Reference two seperate cases."]
#[poise::command(
    slash_command,
    category = "Moderation",
    default_member_permissions = "MANAGE_GUILD"
)]
pub async fn reference(
    ctx: Context<'_>,
    #[description = "The case to assign a reference."] case: u32,
    #[description = "The reference case"] reference: u32,
) -> Result<(), Error> {
    let database = &ctx.data().database();
    let guild_id = ctx.guild_id().unwrap().0;
    let db = GuildDb::new(database, guild_id.to_string()).await;

    let case_1 = db.cases.iter().find(|c| c.id == case);
    let case_2 = db.cases.iter().find(|c| c.id == reference);

    if case_1.is_none() & case_2.is_none() {
        ctx.send(Valeriyya::reply("Cases with this ids don't exist").ephemeral(true)).await;
        return Ok(());
    } else if case_1.is_none() {
        ctx.send(Valeriyya::reply(format!("Case with the id: {} doesn't exist", case)).ephemeral(true)).await;
        return Ok(());
    } else if case_2.is_none() {
        ctx.send(Valeriyya::reply(format!("Case with the id: {} doesn't exist", reference)).ephemeral(true)).await;
        return Ok(());
    }

    update_case(
        database,
        guild_id.to_string(),
        case,
        CaseUpdateAction::Reference,
        CaseUpdateValue {
            reason: None,
            reference: Some(reference),
        },
    );

    if db.channels.logs.is_some() {
        let channel = ChannelId(db.channels.logs.unwrap().parse::<NonZeroU64>().unwrap());
        if case_1.unwrap().message.is_some() {
            let case_found = case_1.unwrap();
            let mut log_channel_msg = channel
                .message(
                    ctx.discord(),
                    MessageId(
                        case_found
                            .message
                            .as_deref()
                            .unwrap()
                            .parse::<NonZeroU64>()
                            .unwrap(),
                    ),
                )
                .await?;
            let staff_user_cache = UserId(case_found.staff_id.parse::<NonZeroU64>().unwrap())
                .to_user(ctx.discord())
                .await?
                .to_owned();
            let staff_user = (
                staff_user_cache.tag(),
                staff_user_cache.id,
                staff_user_cache.face(),
            );
            let target_user = UserId(case_found.target_id.parse::<NonZeroU64>().unwrap())
                .to_user(ctx.discord())
                .await?
                .tag();

            let mut embed = Valeriyya::embed()
                .timestamp(Timestamp::from(&Timestamp::from_unix_timestamp(case_found.date).unwrap()))
                .author(Valeriyya::reply_author(format!("{} ({})", staff_user.0, staff_user.1)).icon_url(staff_user.2))
                .footer(Valeriyya::reply_footer(format!("Case {}", case_found.id)));
            
            if case_found.action == ActionTypes::Mute {
                embed = embed.description(format!(
                    "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nExpiration: {:?}\nReference: `{}`",
                    target_user, case_found.action, case_found.reason, case_found.expiration.unwrap(), &reference
                ));
            } else {
                embed = embed.description(format!(
                    "Member: `{}`\nAction: `{:?}`\nReason: `{}`\nReference: `{}`",
                    target_user, case_found.action, case_found.reason, &reference
                ));
            }
            

            log_channel_msg.edit(ctx.discord(), Valeriyya::msg_edit().embed(embed)).await;
        };
    }

    ctx.send(Valeriyya::reply(format!("Updated case with the id: {case}")).ephemeral(true)).await;
    Ok(())
}
