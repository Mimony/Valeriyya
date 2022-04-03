use crate::{serenity, Context, Error, database::access_guild};

#[derive(poise::ChoiceParameter)]
pub enum OptionChoices {
    #[name = "show"] Show,
    #[name = "delete"] Delete
}


#[poise::command(slash_command, category = "Moderation")]
pub async fn case(
    ctx: Context<'_>,
    #[description = "What to do with the case."] option: OptionChoices,
    #[description = "The id of the case."] id: u64,
) -> Result<(), Error> {

    let database = access_guild(&ctx.data().db_client, ctx.guild_id().unwrap().0.to_string()).await?.unwrap();

    // if let OptionChoices::Show = option {
    //     let case = ctx.cases().get(id).await?;
    //     ctx.send(|s| {
    //         s.embed(|e| {
    //             e.color(serenity::Color::from_rgb(82, 66, 100));
    //             e.author(|a| {
    //                 a.name(format!("{} ({})", case.user.tag(), case.user.id));
    //                 a.icon_url(case.user.face())
    //             });
    //             e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
    //             e.description(format!(
    //                 "Member: `{}`\nAction: `{}`\nReason: {}\nExpiration: {}",
    //                 case.user.tag(),
    //                 case.action,
    //                 case.reason,
    //                 case.expiration
    //             ));
    //             e.timestamp(serenity::Timestamp::now())
    //         });
    //         s.ephemeral(true)
    //     })
    //     .await;
    // } else if let OptionChoices::Delete = option {
    //     let case = ctx.cases().delete(id).await?;
    //     ctx.send(|s| {
    //         s.embed(|e| {
    //             e.color(serenity::Color::from_rgb(82, 66, 100));
    //             e.author(|a| {
    //                 a.name(format!("Case {}", id));
    //                 a.icon_url(ctx.author().face())
    //             });
    //             e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
    //             e.description(format!("Case {} has been deleted.", id));
    //             e.timestamp(serenity::Timestamp::now())
    //         });
    //         s.ephemeral(true)
    //     })
    //     .await;
    // }

    Ok(())
}