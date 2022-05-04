use poise::{
    async_trait,
    serenity_prelude::{ChannelId, Color, Http},
};

use crate::{regex, Context, Error};

use songbird::{
    input::{self, restartable::Restartable, Metadata},
    Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent,
};

#[poise::command(prefix_command, slash_command, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The url of the song"]
    #[rest]
    url: String,
) -> Result<(), Error> {
    let video_id_rgx = regex!(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]");

    let url = video_id_rgx
        .find(&url)
        .map(|u| u.as_str().to_owned())
        .unwrap_or_else(|| format!("ytsearch1:{}", url.trim()));

    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.send(|m| m.content("Not in a voice channel").ephemeral(true))
                .await;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx.discord()).await.unwrap().clone();

    ctx.say("Loading song...").await;
    manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        println!("handler initiated");
        let mut handler = handler_lock.lock().await;

        let source = if handler.queue().len() > 0 {
            match Restartable::ytdl(url, false).await {
                Ok(source) => source,
                Err(e) => {
                    println!("There was an error with making the source: {e}");

                    ctx.send(|m| {
                        m.content("There was a issue with initializing the source")
                            .ephemeral(true)
                    })
                    .await;

                    return Ok(());
                }
            }
            .into()
        } else {
            match input::ytdl(url).await {
                Ok(source) => source,
                Err(e) => {
                    println!("There was an error with making the source: {e}");

                    ctx.send(|m| {
                        m.content("There was a issue with initializing the source")
                            .ephemeral(true)
                    })
                    .await;

                    return Ok(());
                }
            }
        };

        println!("source is done");

        let queue = handler.enqueue_source(source);
        let metadata = queue.metadata().clone();

        let _ = queue.add_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                chan_id: ctx.channel_id(),
                http: ctx.discord().http.clone(),
                metadata: metadata.clone(),
            },
        );

        ctx.send(|m| {
            m.embed(|e| {
                e.color(Color::from_rgb(82, 66, 100))
                    .description(format!(
                        "Playing [{}]({})",
                        metadata.title.unwrap(),
                        metadata.source_url.unwrap()
                    ))
                    .timestamp(poise::serenity_prelude::Timestamp::now())
                    .title("Song start")
            })
        })
        .await;
    }

    Ok(())
}

struct SongEndNotifier {
    chan_id: ChannelId,
    http: std::sync::Arc<Http>,
    metadata: Metadata,
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        self.chan_id
            .send_message(&self.http, |m| {
                m.add_embed(|e| {
                    e.color(Color::from_rgb(82, 66, 100))
                        .description(format!(
                            "{} has ended",
                            self.metadata.title.clone().unwrap()
                        ))
                        .title("Song ended")
                        .timestamp(poise::serenity_prelude::Timestamp::now())
                })
            })
            .await;

        None
    }
}
