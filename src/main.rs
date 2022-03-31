#![feature(fn_traits)]
#![feature(once_cell)]
#![allow(unused_must_use)]

macro_rules! import {
    [ $($cmd:ident), * ] => {
      $(
        mod $cmd;
        pub use $cmd::$cmd;
      )*
    }
}

macro_rules! ternary {
    ($condition:expr => { $true_condition:expr; $false_condition:expr; }) => {
        if $condition {
            $true_condition
        }
        else {
            $false_condition
        }
    }
}

mod commands;
mod database;

use mongodb::{options::{ClientOptions, ResolverConfig}, Client};
use poise::serenity_prelude::{self as serenity, RoleId};
use regex::Regex;

pub fn string_to_sec(raw_text: impl ToString) -> i64 {
    let re = Regex::new(r"((?P<years>\d+?)Y|y|years)?((?P<months>\d+?)months)?((?P<weeks>\d+?)W|w|weeks)?((?P<days>\d+?)D|d|days)?((?P<hours>\d+?)H|h|hr|hours)?((?P<minutes>\d+?)m|min|minutes)?((?P<seconds>\d+?)S|s|sec|seconds)?").unwrap();

    let text = raw_text.to_string();

    let captures = if let Some(caps) = re.captures(&text) {
        caps
    } else {
        return 0;
    };

    let mut seconds = 0;
    for name in [
        "years", "months", "weeks", "days", "hours", "minutes", "seconds",
    ] {
        if let Some(time) = captures.name(name) {
            let time: i64 = time.as_str().parse().unwrap();

            seconds += match name {
                "years" => time * 31_557_600,
                "months" => time * 2_592_000,
                "weeks" => time * 604_800,
                "days" => time * 86_400,
                "hours" => time * 3_600,
                "minutes" => time * 60,    
                "seconds" => time,
                _ => 0,
            };

        } else {
            continue;
        }
    }

    seconds
}


pub async fn get_guild_member(ctx: Context<'_>) -> Result<Option<serenity::Member>, Error> {
	Ok(match ctx.guild_id() {
		Some(guild_id) => Some(guild_id.member(ctx.discord(), ctx.author()).await?),
		None => None,
	})
}

pub async fn get_guild_permissions(ctx: Context<'_>) -> Result<Option<serenity::Permissions>, Error> {
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

pub async fn member_managable(ctx: Context<'_> ,member: &serenity::Member) -> bool {

    let guild = ctx.guild().unwrap();
    if member.user.id == guild.owner_id {
        return false;
    }
    if member.user.id == ctx.discord().cache.current_user_id() {
        return false;
    }
    if ctx.discord().cache.current_user_id() == guild.owner_id {
        return true;
    }

    let me = guild.member(ctx.discord(), ctx.discord().cache.current_user_id()).await.unwrap();

    let highest_me_role: RoleId;
    let member_highest_role: RoleId;


    ternary!(me.roles.len() == 0 => {
        highest_me_role = RoleId(guild.id.0);
        highest_me_role = me.highest_role_info(&ctx.discord().cache).unwrap().0; 
    });

    ternary!(member.roles.len() == 0 => {
        member_highest_role = RoleId(guild.id.0);
        member_highest_role = member.highest_role_info(&ctx.discord().cache).unwrap().0;
    });

    if compare_role_position(ctx, highest_me_role, member_highest_role) > 0 {
        return true;
    } else {
        return false;
    }
}

pub fn compare_role_position(ctx: Context<'_>, role1: serenity::RoleId, role2: serenity::RoleId) -> i64 {
    let guild = ctx.guild().unwrap();
    
    let r1 = guild.roles.get(&role1).unwrap();
    let r2 = guild.roles.get(&role2).unwrap();

    if r1.position == r2.position {
        return i64::from(r2.id) - i64::from(r1.id)
    }

    r1.position - r2.position
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Data {
    database: Client,
    client_id: serenity::UserId
}

async fn init() -> Result<(), Error> {
    let discord_token = "PROVIDE TOKEN HERE";
    let database_url = "PROVIDE MONGODB URL";

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::info::user(),
            commands::moderation::ban(),
            commands::moderation::kick(),
            commands::moderation::mute(),
        ],
        ..Default::default()
    };

    let database_options = ClientOptions::parse_with_resolver_config(&database_url, ResolverConfig::cloudflare()).await?;
    let database = Client::with_options(database_options)?;
    
    poise::Framework::build()
        .token(discord_token)
        .user_data_setup(move |ctx, client, _framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::watching("the lovely moon")).await;
                Ok(Data {
                    database,
                    client_id: client.user.id
                })
            })
        })
        .options(options)
        .client_settings(move |client_builder| {
            client_builder.intents(
                serenity::GatewayIntents::non_privileged()
                    | serenity::GatewayIntents::GUILD_MEMBERS
                    | serenity::GatewayIntents::GUILD_MESSAGES,
            )
        })
        .run()
        .await?;
    Ok(())
}
#[tokio::main]
async fn main() {
    if let Err(e) = init().await {
        println!("{}", e);
        std::process::exit(1);
    }
}