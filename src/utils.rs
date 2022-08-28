#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bson::doc;
use mongodb::Database;
use poise::{
    async_trait,
    serenity_prelude::{ChannelId, Http, RoleId, CreateMessage, CreateEmbed},
};
use serde::{Deserialize, Serialize};
use songbird::{Event, EventContext, EventHandler};

use crate::{serenity, Context, Error};

#[macro_export]
macro_rules! import {
    [ $($cmd:ident), * ] => {
      $(
        mod $cmd;
        pub use $cmd::$cmd;
      )*
    }
}

#[macro_export]
macro_rules! ternary {
    ($condition:expr => { $true_condition:expr; $false_condition:expr; }) => {
        if $condition {
            $true_condition
        } else {
            $false_condition
        }
    };
}

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: ::once_cell::sync::OnceCell<::regex::Regex> = ::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| ::regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! regex_lazy {
    ($re:literal $(,)?) => {
        ::once_cell::sync::Lazy::<::regex::Regex>::new(|| regex::Regex::new($re).unwrap())
    };
}

pub fn valeriyya_embed() -> CreateEmbed {
    CreateEmbed::new()
    .color(PURPLE_COLOR)
    .timestamp(serenity::Timestamp::now())
}

pub fn string_to_sec(raw_text: impl ToString) -> i64 {
    let re = regex_lazy!(
        r"((?P<years>\d+?)\s??y|year|years)?((?P<months>\d+?)\s??month|months)?((?P<weeks>\d+?)\s??w|week|weeks)?((?P<days>\d+?)\s??d|day|days)?((?P<hours>\d+?\s??)h|hour|hours)?((?P<minutes>\d+?)\s??m|min|minutes)?((?P<seconds>\d+?)\s??s|sec|second|seconds)?"
    );

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

pub async fn get_guild_permissions(
    ctx: Context<'_>,
) -> Result<Option<serenity::Permissions>, Error> {
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

pub async fn member_managable(ctx: Context<'_>, member: &serenity::Member) -> bool {
    
    {
        let guild = ctx.guild().unwrap();
        if member.user.id == guild.owner_id {
            return false;
        }
        if member.user.id == ctx.discord().cache.current_user().id {
            return false;
        }
        if ctx.discord().cache.current_user().id == guild.owner_id {
            return true;
        }
    }
    
    let guild_id = ctx.guild_id().unwrap();
    {   
        let user_id = ctx.discord().cache.current_user().id;
        let me = guild_id
            .member(ctx.discord(), user_id)
            .await
            .unwrap();

        #[allow(clippy::len_zero)]
        let highest_me_role: RoleId = ternary!(me.roles.len() == 0 => {
            RoleId(guild_id.0); 
            me.highest_role_info(&ctx.discord().cache).unwrap().0;
        });

        #[allow(clippy::len_zero)]
        let member_highest_role: RoleId = ternary!(member.roles.len() == 0 => {
            RoleId(guild_id.0);
            member.highest_role_info(&ctx.discord().cache).unwrap().0;
        });

        compare_role_position(ctx, highest_me_role, member_highest_role) > 0
    }
}

pub fn compare_role_position(
    ctx: Context<'_>,
    role1: serenity::RoleId,
    role2: serenity::RoleId,
) -> i64 {
    let guild = ctx.guild().unwrap();

    let r1 = guild.roles.get(&role1).unwrap();
    let r2 = guild.roles.get(&role2).unwrap();

    if r1.position == r2.position {
        return i64::from(r2.id) - i64::from(r1.id);
    }

    r1.position - r2.position
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelStruct {
    pub logs: Option<String>,
    pub welcome: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleStruct {
    pub staff: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionTypes {
    ban,
    unban,
    kick,
    mute,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub id: u32,
    pub action: ActionTypes,
    pub guild_id: String,
    pub staff_id: String,
    pub target_id: String,
    pub date: i64,
    pub reason: String,
    pub reference: Option<u32>,
    pub expiration: Option<i64>,
    pub message: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub id: String,
    pub ban: u16,
    pub kick: u16,
    pub mute: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildDb {
    pub gid: String,
    pub cases: Vec<Case>,
    pub cases_number: u32,
    pub history: Vec<History>,
    pub channels: ChannelStruct,
    pub roles: RoleStruct,
}

pub async fn get_guild_db(database: &Database, gid: impl ToString) -> GuildDb {
    let db = database.collection::<GuildDb>("guild");

    let db_guild = db
        .find_one(doc! { "gid": gid.to_string() }, None)
        .await
        .unwrap();
    match db_guild {
        Some(gdb) => gdb,
        None => {
            let doc = GuildDb {
                gid: gid.to_string(),
                cases: Vec::<Case>::new(),
                cases_number: 0,
                channels: ChannelStruct {
                    logs: None,
                    welcome: None,
                },
                roles: RoleStruct { staff: None },
                history: Vec::<History>::new(),
            };
            let id = db.insert_one(doc, None).await.unwrap();
            db.find_one(
                doc! {
                    "_id": id.inserted_id
                },
                None,
            )
            .await
            .unwrap()
            .unwrap()
        }
    }
}

pub async fn update_guild_db(database: &Database, gid: impl ToString, value: &GuildDb) -> GuildDb {
    let db = database.collection::<GuildDb>("guild");

    db.find_one_and_update(
        doc! { "gid": gid.to_string() },
        doc! {
            "$set": bson::to_document(value).unwrap()
        },
        None,
    )
    .await
    .unwrap()
    .unwrap()
}

pub async fn create_case(database: &Database, gid: impl ToString, case: Case) {
    let mut db = get_guild_db(database, gid.to_string()).await;

    db.cases.push(case);
    db.cases_number += 1;
    update_guild_db(database, gid, &db).await;
}

pub enum CaseUpdateAction {
    reason,
    reference,
}

pub struct CaseUpdateValue {
    pub reason: Option<String>,
    pub reference: Option<u32>,
}

pub async fn update_case(
    database: &Database,
    gid: impl ToString,
    id: u32,
    action: CaseUpdateAction,
    value: CaseUpdateValue,
) {
    let mut db = get_guild_db(database, gid.to_string()).await;

    let mut c = db.cases.iter_mut().find(|c| c.id == id).unwrap();

    if let CaseUpdateAction::reason = action {
        c.reason = value.reason.unwrap();
    } else {
        c.reference = Some(value.reference.unwrap());
    }

    update_guild_db(database, gid, &db).await;
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseVideoApi {
    pub kind: String,
    pub etag: String,
    pub nextPageToken: String,
    pub regionCode: String,
    pub pageInfo: PageInfo,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PageInfo {
    pub totalResults: u32,
    pub resultsPerPage: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    pub kind: String,
    pub etag: String,
    pub id: ItemId,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemId {
    pub kind: String,
    pub videoId: String,
}

#[derive(Clone, Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub duration: std::time::Duration,
}

pub struct SongEndNotifier {
    pub chan_id: ChannelId,
    pub http: std::sync::Arc<Http>,
    pub metadata: Video,
}

pub struct SongPlayNotifier {
    pub chan_id: ChannelId,
    pub http: std::sync::Arc<Http>,
    pub metadata: Video,
}

#[async_trait]
impl EventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        self.chan_id
            .send_message(&self.http, CreateMessage::new()
                .add_embed( CreateEmbed::new()
                    .color(PURPLE_COLOR)
                        .description(format!("{} has ended", self.metadata.title))
                        .title("Song ended")
                        .timestamp(poise::serenity_prelude::Timestamp::now())
                )
            )
            .await;

        None
    }
}

#[async_trait]
impl EventHandler for SongPlayNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        self.chan_id
            .send_message(&self.http, CreateMessage::default()
                .add_embed(CreateEmbed::new()
                    .color(PURPLE_COLOR)
                        .description(format!(
                            "Playing [{}]({})",
                            self.metadata.title,
                            format_args!("https://youtu.be/{}", self.metadata.id)
                        ))
                        .title("Song playing")
                        .timestamp(poise::serenity_prelude::Timestamp::now())
                )
            )
            .await;

        None
    }
}

pub const PURPLE_COLOR: serenity::Color = serenity::Color::from_rgb(82, 66, 100);
