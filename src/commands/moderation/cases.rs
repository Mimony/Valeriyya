use crate::{
    serenity,
    utils::{get_guild_db, get_guild_member, update_guild_db, ActionTypes},
    Context, Error,
};

#[derive(poise::ChoiceParameter)]
pub enum OptionChoices {
    #[name = "show"]
    Show,
    #[name = "delete"]
    Delete,
}

#[poise::command(prefix_command, slash_command, category = "Moderation")]
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
            ctx.send(|m| {
                m.content(format!("Can't find a case with the id: {}", id))
                    .ephemeral(true)
            })
            .await;
            return Ok(());
        }

        let case = case.unwrap();

        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100));
                e.author(|a| {
                    a.name(format!("{} ({})", staff.user.tag(), staff.user.id));
                    a.icon_url(staff.user.face())
                });
                e.thumbnail(ctx.guild().unwrap().icon_url().unwrap());
                if ActionTypes::mute == case.action {
                    e.description(format!(
                        "Member: `{}`\nAction: `{:?}`\nReason: {}\nExpiration:<t:{}:R>",
                        case.target_id, case.action, case.reason, case.date
                    ));
                } else {
                    e.description(format!(
                        "Member: `{}`\nAction: `{:?}`\nReason: {}\n",
                        case.target_id, case.action, case.reason
                    ));
                }
                e.timestamp(serenity::Timestamp::now())
                .footer(|f| {
                    f.text(format!("Case {}", case.id))
                })
            });
            s.ephemeral(true)
        })
        .await;
    } else if let OptionChoices::Delete = option {
        let case = db.cases.iter().find(|c| c.id == id);

        if case.is_none() {
            ctx.send(|m| {
                m.content(format!("Can't find a case with the id: {}", id))
                    .ephemeral(true)
            })
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

        ctx.send(|s| {
            s.embed(|e| {
                e.color(serenity::Color::from_rgb(82, 66, 100));
                e.author(|a| {
                    a.name(format!("{} ({})", staff.user.tag(), staff.user.id));
                    a.icon_url(staff.user.face())
                });
                e.description(format!("Case with the id: {} has been deleted.", id));
                e.timestamp(serenity::Timestamp::now())
            });
            s.ephemeral(true)
        })
        .await;
    }

    Ok(())
}
