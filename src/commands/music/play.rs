use poise::{
    async_trait,
    serenity_prelude::{ChannelId, Color, Http},
};

use crate::{Context, Error};

use songbird::{
    input::YoutubeDl, Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent,
};

/// Plays a song
#[poise::command(prefix_command, slash_command, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The url of the song"]
    #[rest]
    url: String,
) -> Result<(), Error> {
    let video_id_rgx = crate::regex!(r"[0-9A-Za-z_-]{10}[048AEIMQUYcgkosw]");

    let url = video_id_rgx
        .find(&url)
        .map(|u| u.as_str().to_owned())
        .unwrap_or_else(|| format!("ytsearch1:{}", url.trim()));
    let ytextract = ytextract::Client::new();

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

    let msg = ctx.say("Loading song...").await.unwrap();
    manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        println!("handler initiated");
        let mut handler = handler_lock.lock().await;

        let metadata = ytextract.video(url.clone().parse()?).await?;
        let source = YoutubeDl::new(reqwest::Client::new(), url);

        println!("source is done");

        let queue = handler.enqueue(source.into()).await;

        println!("queued and adding the event");

        let _ = queue.add_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                chan_id: ctx.channel_id(),
                http: ctx.discord().http.clone(),
            },
        );

        let playing_status = crate::ternary!(handler.queue().is_empty() => {
            "Playing";
            "Queued";
        });
        
        msg.edit(ctx, |m| {
            m.embed(|e| {
                e.color(Color::from_rgb(82, 66, 100))
                    .description(format_args!(
                        "{playing_status} [{}]({})",
                        metadata.title(),
                        // "temp title",
                        // "temp url",
                        format_args!("https://youtu.be/{}", metadata.id()) // metadata.source_url.unwrap()
                    ))
                    .timestamp(poise::serenity_prelude::Timestamp::now())
                    .title("Song start")
            })
        })
        .await?;
    } else {
        ctx.send(|m| {
            m.content("Join a voice channel and then try that again!")
                .ephemeral(true)
        })
        .await?;
    }

    Ok(())
}

struct SongEndNotifier {
    chan_id: ChannelId,
    http: std::sync::Arc<Http>,
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
                            // self.metadata.title.clone().unwrap()
                            "temporary song name"
                        ))
                        .title("Song ended")
                        .timestamp(poise::serenity_prelude::Timestamp::now())
                })
            })
            .await;

        None
    }
}
