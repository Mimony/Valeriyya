// #![allow(dead_code)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

use mongodb::Database;
use poise::{
    serenity_prelude::{
        Color, CreateEmbed, Member, Permissions, Role, RoleId, Timestamp, UserId, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, EditMessage,
    }, CreateReply,
};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use iso8601_duration::Duration as iso_duration;

use crate::{Context, Error, structs::{CaseUpdateAction, CaseUpdateValue, GuildDb, Video, ResponseVideoApi, ResponsePlaylistApi, ResponseSearchVideoApi, SearchVideoItem}};

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

async fn _get_spotify_metadata(url: impl Into<String>, reqwest: &reqwest::Client) {
    let url = format!("https://api.spotify.com/v1/search/{}", url.into());
    reqwest.get(url)
    .header(AUTHORIZATION, "Bearer [AUTH_TOKEN]")
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send().await.unwrap().text();
}

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

fn string_to_sec(raw_text: impl ToString) -> i64 {
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

fn aggregate_role_permissions(
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
    
    let user_id = ctx.discord().cache.current_user().id;
    let me = guild_id
        .member(ctx.discord(), user_id)
        .await
        .unwrap();

    let highest_me_role: RoleId = if me.roles.is_empty() {
        RoleId(guild_id.0)
    } else {
       me.highest_role_info(&ctx.discord().cache).unwrap().0
    };

    let member_highest_role: RoleId = if member.roles.is_empty() {
        RoleId(guild_id.0)
    } else {
        member.highest_role_info(&ctx.discord().cache).unwrap().0
    };

    compare_role_position(ctx, highest_me_role, member_highest_role) > 0
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

pub async fn update_case(
    database: &Database,
    gid: String,
    id: u32,
    action: CaseUpdateAction,
    value: CaseUpdateValue,
) {
    let mut db = Valeriyya::get_database(database, gid).await;

    let mut c = db.cases.iter_mut().find(|c| c.id == id).unwrap();

    if let CaseUpdateAction::Reason = action {
        c.reason = value.reason.unwrap();
    } else if let CaseUpdateAction::Reference = action {
        c.reference = Some(value.reference.unwrap());
    }

    db.execute(database).await;
}

pub const PURPLE_COLOR: Color = Color::from_rgb(82, 66, 100);

pub struct Valeriyya;

impl Valeriyya {
    // * Shortcuts to most Create structures
    pub fn embed() -> CreateEmbed {
        CreateEmbed::default()
            .color(PURPLE_COLOR)
            .timestamp(Timestamp::now())
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
    pub fn ms(raw_text: impl ToString) -> i64 {
        string_to_sec(raw_text)
    }

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