use crate::{
    utils::{Valeriyya, ResponseVideoApi, SongEndNotifier, SongPlayNotifier, Video},
    Context, Error,
};
use futures::StreamExt;

use songbird::{input::YoutubeDl, Event, TrackEvent};
use std::time::Duration;

/// Plays a song.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Music",
    aliases("p"),
    default_member_permissions = "VIEW_CHANNEL"
)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The url of the song"]
    #[rest]
    url: String,
) -> Result<(), Error> {
    let video_id_regex = crate::regex!(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]");
    let playlist_id_regex =
        crate::regex!(r"(?:(?:PL|LL|EC|UU|FL|RD|UL|TL|PU|OLAK5uy_)[0-9A-Za-z-_]{10,}|RDMM)");

    let request_client = reqwest::Client::new();

    let url: (String, bool) = match playlist_id_regex
        .find(&url)
        .map(|u| u.as_str().to_owned()) {
            Some(url) => (url, false),
            None => {
                match video_id_regex
                .find(&url)
                .map(|u| u.as_str().to_owned()) {
                    Some(url) => (url, true),
                    None => {
                        (request_client.get(
                            format!(
                                "https://youtube.googleapis.com/youtube/v3/search?part=snippet&order=relevance&type=video&maxResults=1&q={query}&key={api_key}", 
                                query = url.clone(), api_key = ctx.data().api_key
                            ))
                            .send().await?.json::<ResponseVideoApi>().await?.items[0].id.videoId.clone(), true)

                    }
                }
            },
        };
    let ytextract = ytextract::Client::new();

    let guild_id = ctx.guild_id().unwrap();

    let channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send(Valeriyya::reply("You are not in a voice channel").ephemeral(true)).await;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    let msg = ctx.say("Loading song...").await.unwrap();
    manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let metadata: (Vec<Video>, bool) = match url.1 {
            true => {
                let video = ytextract.video(url.0.parse()?).await?;
                (
                    vec![Video {
                        id: video.id().to_string(),
                        duration: video.duration(),
                        title: video.title().to_string(),
                    }],
                    true,
                )
            }
            false => {
                let playlist_data = ytextract.playlist(url.0.parse()?).await?;
                let videos = playlist_data.videos();
                futures::pin_mut!(videos);
                let mut vec: Vec<Video> = vec![];

                while let Some(video) = videos.next().await {
                    let video = match video {
                        Ok(vid) => vid,
                        Err(_) => {
                            continue;
                        }
                    };

                    let video = ytextract.video(video.id()).await?;
                    vec.push(Video {
                        id: video.id().to_string(),
                        duration: video.duration(),
                        title: video.title().to_string(),
                    });
                }
                (vec, false)
            }
        };

        let source: Vec<YoutubeDl> = match metadata.1 {
            true => {
                vec![YoutubeDl::new(request_client, metadata.0[0].id.parse()?)]
            }
            false => {
                let vids = &metadata.0;
                let mut yt: Vec<YoutubeDl> = Vec::new();
                for q in vids {
                    yt.push(YoutubeDl::new(request_client.clone(), q.id.parse()?));
                }
                yt
            }
        };

        for (i, s) in source.into_iter().enumerate() {
            let video_bool = &metadata.1;
            let metadata = &metadata.0;
            let queue = handler.enqueue_with_preload(
                s.into(),
                Some(metadata[i].duration - Duration::from_secs(15)),
            );
            let _ = queue.add_event(
                Event::Track(TrackEvent::End),
                SongEndNotifier {
                    chan_id: ctx.channel_id(),
                    http: ctx.discord().http.clone(),
                    metadata: metadata[i].clone(),
                },
            );

            #[allow(clippy::if_same_then_else)]
            if !video_bool && i >= 1 {
                let _ = queue.add_event(
                    Event::Track(TrackEvent::Play),
                    SongPlayNotifier {
                        chan_id: ctx.channel_id(),
                        http: ctx.discord().http.clone(),
                        metadata: metadata[i].clone(),
                    },
                );
            } else if handler.queue().len() >= 2 {
                let _ = queue.add_event(
                    Event::Track(TrackEvent::Play),
                    SongPlayNotifier {
                        chan_id: ctx.channel_id(),
                        http: ctx.discord().http.clone(),
                        metadata: metadata[i].clone(),
                    },
                );
            };
        }

        let queue_clone = handler.queue().clone();
        let mng = manager.clone();

        tokio::task::spawn(async move {
            let queue = queue_clone;

            loop {
                if !queue.is_empty() {
                    tokio::time::sleep(Duration::from_secs(600)).await;
                    continue;
                }
                mng.remove(guild_id).await;
                break;
            }
        });

    
        msg.edit(ctx, Valeriyya::reply("").embed(
            Valeriyya::embed()
                .description(format!(
                    "Queued [{}]({})",
                    metadata.0[0].title,
                    format_args!("https://youtu.be/{}", metadata.0[0].id)
                ))
                .title("Song playing")
        )).await?;

        drop(handler);
    };

    Ok(())
}
