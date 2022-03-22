#![feature(fn_traits)]
#![allow(unused_must_use)]
#![allow(unused_macros)]


mod commands;


use poise::serenity_prelude as serenity;

async fn get_guild_member(ctx: Context<'_>) -> Result<Option<serenity::Member>, Error> {
	Ok(match ctx.guild_id() {
		Some(guild_id) => Some(guild_id.member(ctx.discord(), ctx.author()).await?),
		None => None,
	})
}

async fn get_guild_permissions(ctx: Context<'_>) -> Result<Option<serenity::Permissions>, Error> {
	fn aggregate_role_permissions(
		guild_member: &serenity::Member,
		guild_owner_id: serenity::UserId,
		guild_roles: &std::collections::HashMap<serenity::RoleId, serenity::Role>,
	) -> serenity::Permissions {
		if guild_owner_id == guild_member.user.id {
			serenity::Permissions::all()
		} else {
			guild_member
				.roles
				.iter()
				.filter_map(|r| guild_roles.get(r))
				.fold(serenity::Permissions::empty(), |a, b| a | b.permissions)
		}
	}

	if let (Some(guild_member), Some(guild_id)) = (get_guild_member(ctx).await?, ctx.guild_id()) {
		let permissions = if let Some(guild) = guild_id.to_guild_cached(&ctx.discord()) {
			aggregate_role_permissions(&guild_member, guild.owner_id, &guild.roles)
		} else {
			let guild = &guild_id.to_partial_guild(&ctx.discord()).await?;
			aggregate_role_permissions(&guild_member, guild.owner_id, &guild.roles)
		};

		Ok(Some(permissions))
	} else {
		Ok(None)
	}
}

pub async fn member_managable(ctx: Context<'_> ,member: serenity::Member) -> bool {

    let guild = ctx.guild().unwrap();
    if member.user.id == guild.owner_id {
        return false
    }
    if member.user.id == ctx.discord().cache.current_user_id() {
        return false
    }
    if ctx.discord().cache.current_user_id() == guild.owner_id {
        return true
    }

    // let me = guild.member(ctx.discord(), ctx.discord().cache.current_user_id()).await.unwrap();
    // let highest_me_role = me.highest_role_info(&ctx.discord().cache).unwrap().0;

    false
    // if member.highest_role_info(&ctx.discord().cache).unwrap().0 == highest_me_role {
    //     return false
    // }
}

// async fn compare_role_position(ctx: Context<'_>, role1: serenity::RoleId, role2: serenity::RoleId) {
//     let guild = ctx.guild().unwrap();
    
// }

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Data {
    bot_user_id: serenity::UserId,
    bot_start_time: std::time::Instant,
    // database: sqlx::Postgres,
}

async fn app() -> Result<(), Error> {
    let discord_token = "ODMwMTMwMzAxNTM1NjQ5ODUz.YHCNFg.FwlkE2je_AAfw5gIn2qn0EO3Vuc";

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::info::user::user(),
        ],
        ..Default::default()
    };

    poise::Framework::build()
        .token(discord_token)
        .user_data_setup(move |ctx, bot, _framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::playing("development")).await;
                Ok(Data {
                    bot_user_id: bot.user.id,
                    bot_start_time: std::time::Instant::now(),
                })
            })
        })
        .options(options)
        .client_settings(move |client_builder| {
            client_builder.intents(
                serenity::GatewayIntents::non_privileged()
                    | serenity::GatewayIntents::GUILD_MEMBERS,
            )
        })
        .run()
        .await?;

    Ok(())
}
#[tokio::main]
async fn main() {
    if let Err(e) = app().await {
        println!("{}", e);
        std::process::exit(1);
    }
}