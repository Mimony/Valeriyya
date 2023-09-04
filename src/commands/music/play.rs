use crate::{
    structs::{SongEndNotifier, SongPlayNotifier, Video},
    utils::Valeriyya,
    Context, Error,
};

use songbird::{input::YoutubeDl, Event, TrackEvent};
use std::time::Duration;

#[doc = "Plays a song."]
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
    let video_bool = crate::regex!(r"(?:(?:PL|LL|EC|UU|FL|RD|UL|TL|PU|OLAK5uy_)[0-9A-Za-z-_]{10,}|RDMM)").find(&url).is_none();
    let request_client = reqwest::Client::new();
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
            ctx.send(Valeriyya::reply("You are not in a voice channel..").ephemeral(true)).await?;
            return Ok(());
        }
    };

    let msg = ctx.say("Loading song...").await.unwrap();
    let _ = ctx.data().songbird.join(guild_id, connect_to).await;

    if let Some(handler_lock) = ctx.data().songbird.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let metedata_url = url.clone();
        let metadata: Vec<Video> = match video_bool {
            true => Valeriyya::get_video_metadata(ctx, metedata_url).await,
            false => Valeriyya::get_playlist_metadata(ctx, metedata_url).await
        };

        let source = match video_bool {
            true => vec![YoutubeDl::new(request_client, metadata[0].id.clone())],
            false => {
                let videos = metadata.clone();
                let mut yt: Vec<YoutubeDl> = Vec::with_capacity(100);
                for vid in videos {
                    yt.push(YoutubeDl::new(request_client.clone(), vid.id))
                }
                yt
            }
        };

        for (i, s) in source.into_iter().enumerate() {
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

            if metadata.len() >= 2 && i >= 1 {
                let _ = queue.add_event(
                    Event::Track(TrackEvent::Play),
                    SongPlayNotifier {
                        chan_id: ctx.channel_id(),
                        http: ctx.discord().http.clone(),
                        metadata: metadata[i].clone(),
                    },
                );
            }
        }

        let queue_clone = handler.queue().clone();
        let mng = ctx.data().songbird.clone();

        tokio::task::spawn(async move {
            let queue = queue_clone;

            loop {
                if !queue.is_empty() {
                    tokio::time::sleep(Duration::from_secs(600)).await;
                    continue;
                }
                let _ = mng.remove(guild_id).await;
                break;
            }
        });

        let information_title = match video_bool {
            true => {
                if handler.queue().len() >= 2 {
                    "Queued"
                } else {
                    "Playing"
                }
            },
            false => {
                "Playing"
            }
        };

        msg.edit(ctx, Valeriyya::reply("").embed(
            Valeriyya::embed()
                .description(format!(
                    "{} [{}]({})",
                    information_title,
                    metadata[0].title,
                    format_args!("https://youtu.be/{}", metadata[0].id)
                ))
                .title("Song information")
        )).await?;

        drop(handler);
    };

    Ok(())
}
