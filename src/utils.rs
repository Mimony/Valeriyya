#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bson::doc;
use mongodb::Database;
use poise::{
    async_trait,
    serenity_prelude::{
        ChannelId, Color, CreateEmbed, Http, Member, Permissions, Role, RoleId, Timestamp, UserId, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, EditMessage,
    }, CreateReply,
};
use serde::{Deserialize, Serialize};
use songbird::{Event, EventContext, EventHandler};
use iso8601_duration::Duration as iso_duration;

use crate::{Context, Error};

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

// async fn get_spotify_metadata(url: impl Into<String>, reqwest: &reqwest::Client) {
//     reqwest.get(url)
//     .header(reqwest::Respo, )
// }

async fn search_video(query: impl Into<String>, api_key: &String, reqwest: &reqwest::Client) -> SearchVideoItem  {
    let url = format!(
        "https://youtube.googleapis.com/youtube/v3/search?part=snippet&order=relevance&type=video&maxResults=1&q={}&key={}", 
        query.into(), 
        api_key
    );
    let video = reqwest.get(url).send()
    .await.expect("Error getting Video search.")
    .json::<ResponseSearchVideoApi>()
    .await.expect("Error parsing the Video search JSON.").items;
    video.first().expect("Error getting the first Video search.").clone()
}

async fn get_metadata(ctx: Context<'_>, url: impl Into<String>, playlist: bool) -> Vec<Video>  {
    let url = url.into();
    let reqwest_client = reqwest::Client::new();
    let api_key = ctx.data().api_key.clone();

    let id = if playlist {
        regex!(r"(?:(?:PL|LL|EC|UU|FL|RD|UL|TL|PU|OLAK5uy_)[0-9A-Za-z-_]{10,}|RDMM)").find(&url).map(|u| u.as_str().to_owned()).unwrap()
    } else {
        match regex!(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]").find(&url).map(|u| u.as_str().to_owned()) {
            Some(u) => u,
            None => {
                search_video(url.clone(), &api_key, &reqwest_client).await.id.video_id
            }
        }
    };

    if playlist {
        let request_playlist_url = format!(
            "https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&maxResults=100&playlistId={}&key={}",
            id,
            api_key
        );
        let playlist_items = reqwest_client.get(request_playlist_url)
        .send()
        .await.expect("Error getting Playlist JSON.")
        .json::<ResponsePlaylistApi>()
        .await.expect("Error parsing the Playlist JSON.").items;

        let mut video_ids: Vec<String> = Vec::with_capacity(100);
        for item in playlist_items.into_iter() {
            video_ids.push(item.snippet.resource_id.video_id);
        };

        let request_videos_url = format!(
            "https://youtube.googleapis.com/youtube/v3/videos?part=snippet%2CcontentDetails&id={}&key={}",
            video_ids.join(","),
            api_key
        );

        let video_items = reqwest_client.get(request_videos_url)
        .send()
        .await.expect("Error getting Videos from the Video Id Vector.")
        .json::<ResponseVideoApi>()
        .await.expect("Error parsing the Videos JSON.").items;

        let mut videos: Vec<Video> = Vec::with_capacity(100);
        for item in video_items.into_iter() {
            let duration = iso_duration::parse(&item.content_details.duration).unwrap().to_std();
            videos.push(Video {
                id: item.id,
                title: item.snippet.title,
                duration
            });
        }
        return videos;
    }

    let request_video_url = format!(
        "https://youtube.googleapis.com/youtube/v3/videos?part=snippet%2CcontentDetails&id={}&key={}",
        id,
        api_key
    );

    let item = reqwest_client.get(request_video_url)
    .send()
    .await.expect("Error getting Video JSON")
    .json::<ResponseVideoApi>()
    .await.expect("Error parsing the Video JSON.").items.first().expect("There is no video from this url.").clone();
    let duration = iso_duration::parse(&item.content_details.duration).unwrap().to_std();
    vec![Video {
        id: item.id,
        title: item.snippet.title,
        duration
    }]
}

fn valeriyya_embed() -> CreateEmbed {
    CreateEmbed::default()
    .color(PURPLE_COLOR)
    .timestamp(Timestamp::now())
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

pub async fn get_guild_member(ctx: Context<'_>) -> Result<Option<Member>, Error> {
    Ok(match ctx.guild_id() {
        Some(guild_id) => Some(
            guild_id
                .member(ctx.discord(), ctx.author())
                .await?,
        ),
        None => None,
    })
}

pub fn aggregate_role_permissions(
    guild_member: &Member,
    guild_owner_id: UserId,
    guild_roles: &std::collections::HashMap<RoleId, Role>,
) -> Permissions {
    if guild_owner_id == guild_member.user.id {
        Permissions::all()
    } else {
        guild_member
            .roles
            .iter()
            .filter_map(|r| guild_roles.get(r))
            .fold(Permissions::empty(), |a, b| a | b.permissions)
    }
}
pub async fn get_guild_permissions(ctx: Context<'_>) -> Result<Option<Permissions>, Error> {
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

pub async fn member_managable(ctx: Context<'_>, member: &Member) -> bool {
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

pub fn compare_role_position(ctx: Context<'_>, role1: RoleId, role2: RoleId) -> i64 {
    let guild = ctx.guild().unwrap();

    let r1 = guild.roles.get(&role1).unwrap();
    let r2 = guild.roles.get(&role2).unwrap();

    if r1.position == r2.position {
        return i64::from(r2.id) - i64::from(r1.id);
    }

    (r1.position - r2.position).into()
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GuildDbChannels {
    pub logs: Option<String>,
    pub welcome: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GuildDbRoles {
    pub staff: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionTypes {
    Ban,
    Unban,
    Kick,
    Mute,
}

impl ActionTypes {}

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
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub id: String,
    pub ban: u16,
    pub kick: u16,
    pub mute: u16,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GuildDb {
    pub gid: String,
    pub cases: Vec<Case>,
    pub cases_number: u32,
    pub history: Vec<History>,
    pub channels: GuildDbChannels,
    pub roles: GuildDbRoles,
}

impl GuildDb {
    pub async fn new(db: &Database, guild_id: impl Into<String>) -> Self {
        let guild_id_clone = guild_id.into().clone();
        let db = db.collection::<GuildDb>("guild");
        let db_guild = db
            .find_one(doc! { "gid": guild_id_clone.clone() }, None)
            .await
            .unwrap();

        if let Some(guilddb) = db_guild {
            guilddb
        } else {
            let doc = Self::default().guild_id(guild_id_clone);
            let id = db.insert_one(doc, None).await.unwrap();
            db.find_one(
                doc! {
                    "_id": id.inserted_id,
                },
                None,
            )
            .await
            .unwrap()
            .unwrap()
        }
    }

    #[inline(always)]
    pub fn guild_id(mut self, gid: impl Into<String>) -> Self {
        self.gid = gid.into();
        self
    }

    #[inline(always)]
    pub fn add_cases(mut self, case: Case) -> Self {
        let cases_number = self.cases_number + 1;
        self = self.set_cases_count(cases_number);
        self.cases.push(case);
        self
    }

    #[inline(always)]
    pub fn set_cases(mut self, cases: Vec<Case>) -> Self {
        self.cases = cases;
        self
    }

    #[inline(always)]
    pub fn delete_cases(mut self, index: usize) -> Self {
        self.cases.remove(index);
        self
    }

    #[inline(always)]
    pub fn set_cases_count(mut self, cases_number: u32) -> Self {
        self.cases_number = cases_number;
        self
    }

    #[inline(always)]
    pub fn set_history(mut self, history: Vec<History>) -> Self {
        self.history = history;
        self
    }

    #[inline(always)]
    pub fn set_channels(mut self, channels: GuildDbChannels) -> Self {
        self.channels = channels;
        self
    }

    #[inline(always)]
    pub fn set_roles(mut self, roles: GuildDbRoles) -> Self {
        self.roles = roles;
        self
    }

    pub async fn execute(self, database: &Database) -> Self {
        let db = database.collection::<GuildDb>("guild");
        db.find_one_and_update(
            doc! { "gid": self.gid.clone() },
            doc! {
                "$set": bson::to_document(&self).unwrap()
            },
            None,
        )
        .await
        .unwrap()
        .unwrap()
    }
}

impl GuildDbChannels {
    #[inline(always)]
    pub fn set_logs_channel(mut self, logs: Option<String>) -> Self {
        self.logs = logs;
        self
    }

    #[inline(always)]
    pub fn set_welcome_channel(mut self, welcome: Option<String>) -> Self {
        self.welcome = welcome;
        self
    }
}

impl GuildDbRoles {
    #[inline(always)]
    pub fn set_staff_role(mut self, staff: Option<String>) -> Self {
        self.staff = staff;
        self
    }
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
    gid: String,
    id: u32,
    action: CaseUpdateAction,
    value: CaseUpdateValue,
) {
    let mut db = GuildDb::new(database, gid).await;

    let mut c = db.cases.iter_mut().find(|c| c.id == id).unwrap();

    if let CaseUpdateAction::reason = action {
        c.reason = value.reason.unwrap();
    } else {
        c.reference = Some(value.reference.unwrap());
    }

    db.execute(database).await;
}

#[derive(Deserialize, Debug, Clone)]
struct ResponseSearchVideoApi {
    items: Vec<SearchVideoItem>
}

#[derive(Deserialize, Debug, Clone)]
struct SearchVideoItem {
    id: SearchVideoId
}

#[derive(Deserialize, Debug, Clone)]
struct SearchVideoId {
    #[serde(rename="videoId")]
    video_id: String
}

#[derive(Deserialize, Debug, Clone)]
struct VideoItem {
    id: String,
    snippet: VideoSnippet,
    #[serde(rename="contentDetails")]
    content_details: ContentDetails
}

#[derive(Deserialize, Debug, Clone)]
struct VideoSnippet {
    title: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ContentDetails {
    duration: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ResponseVideoApi {
    items: Vec<VideoItem>
}

#[derive(Deserialize, Debug, Clone)]
struct PlaylistSnippet {
    #[serde(rename="resourceId")]
    resource_id: ResourceId
}

#[derive(Deserialize, Debug, Clone)]
struct ResourceId {
    #[serde(rename="videoId")]
    video_id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct PlaylistItem {
    id: String,
    snippet: PlaylistSnippet,
}

#[derive(Deserialize, Debug, Clone)]
struct ResponsePlaylistApi {
    items: Vec<PlaylistItem>
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
            .send_message(&self.http, Valeriyya::msg_reply().add_embed(
                Valeriyya::embed()
                    .description(format!("{} has finished.", self.metadata.title))
                    .title("Song information")
            )).await;

        None
    }
}

#[async_trait]
impl EventHandler for SongPlayNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        self.chan_id
            .send_message(&self.http, Valeriyya::msg_reply().add_embed(
                Valeriyya::embed()
                    .description(format!(
                        "Playing [{}]({})",
                        self.metadata.title,
                        format_args!("https://youtu.be/{}", self.metadata.id)
                    ))
                    .title("Song information")
            )).await;

        None
    }
}

pub const PURPLE_COLOR: Color = Color::from_rgb(82, 66, 100);

pub struct Valeriyya;

impl Valeriyya {
    // * Shortcuts to most Create structures
    pub fn embed() -> CreateEmbed {
        valeriyya_embed()
    }

    pub fn msg_reply() -> CreateMessage {
        CreateMessage::new()
    }

    pub fn msg_edit() -> EditMessage {
        EditMessage::new()
    }

    pub fn reply_default() -> CreateReply {
        CreateReply::new()
    }

    pub fn reply(content: impl Into<String>) -> CreateReply {
        CreateReply::new().content(content)
    }

    pub fn reply_author(content: impl Into<String>) -> CreateEmbedAuthor {
        CreateEmbedAuthor::new(content)
    }

    pub fn reply_footer(content: impl Into<String>) -> CreateEmbedFooter {
        CreateEmbedFooter::new(content)
    }

    // * Utility functions
    pub async fn get_database(db: &Database, guild_id: impl Into<String>) -> GuildDb {
        GuildDb::new(db, guild_id).await
    }

    pub async fn get_video_metadata(ctx: Context<'_>, url: impl Into<String>) -> Vec<Video> {
        get_metadata(ctx, url, false).await
    }

    pub async fn get_playlist_metadata(ctx: Context<'_>, url: impl Into<String>) -> Vec<Video> {
        get_metadata(ctx, url, true).await
    }

}