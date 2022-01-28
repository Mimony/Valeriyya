import { DiscordGatewayAdapterCreator, joinVoiceChannel, VoiceConnectionStatus } from "@discordjs/voice";
import { GuildMember } from "discord.js";
import { MusicSubscription, Track } from "../../lib/util/valeriyya.music";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";
import { reply } from "../../lib/util/valeriyya.util";
import { waitForResourceToEnterState } from "../../lib/util/valeriyya.music";
import play, { SpotifyAlbum, SpotifyPlaylist } from "play-dl";
import type { Valeriyya } from "../../lib/valeriyya.client";

export default defineCommand({
    data: {
        name: "play",
        description: "Play a song thru the bot.",
        options: [
            {
                name: "song",
                description: "The song that will be played.",
                type: OptionTypes.STRING,
                required: true,
            },
        ],
    },
    // @ts-ignore
    chat: async (int: ICommandInteraction) => {
        let subscription = int.client.subscription.get(int.guildId!);
        console.log(subscription)
        const url = int.options.getString("song")!;
        let song: string | string[];

        const validate = play.yt_validate(url);
        const sp_validate = play.sp_validate(url);

        await int.deferReply();
        
        if (sp_validate === "track") {
            if (play.is_expired()) await play.refreshToken();

            const spotify = await play.spotify(url);
            const searched = await play.search(spotify.name, { limit: 1 });
            song = searched[0].url;
        } else if (sp_validate === "album") {
            if (play.is_expired()) await play.refreshToken();

            const videos_spotify = await (await play.spotify(url) as SpotifyAlbum).all_tracks();
            const videos: string[] = [];
            videos_spotify.forEach(async (vs) => {
                let searched = await play.search(vs.name, { limit: 1 })
                videos.push(searched[0].url)
            })
            song = videos;
        } else if (sp_validate === "playlist") {
            if (play.is_expired()) await play.refreshToken();

            const videos_spotify = await (await play.spotify(url) as SpotifyPlaylist).all_tracks();
            const videos: string[] = [];
            videos_spotify.forEach(async (vs) => {
                let searched = await play.search(vs.name, { limit: 1 })
                videos.push(searched[0].url)
            })
            song = videos;
        } else if (validate === "search") {
            let videos = await play.search(url, { limit: 1 });
            if (videos.length === 0)
                return {
                    content: "I couldn't find the song you were looking for. Please try to search a more specific name.",
                };
            song = videos[0].url;
        } else if (validate === "playlist") {
            const videos = await (await play.playlist_info(url)).all_videos();
            song = videos.map(v => v.url);
            console.log(videos.map(v => v.title))
        } else song = url;

        if (!subscription) {
            if (int.member instanceof GuildMember && int.member.voice.channel) {
                const channel = int.member.voice.channel;
                subscription = new MusicSubscription({ client: int.client as Valeriyya, guildId: int.guildId! },
                    joinVoiceChannel({
                        channelId: channel.id,
                        guildId: channel.guild.id,
                        adapterCreator: int.guild!.voiceAdapterCreator as DiscordGatewayAdapterCreator,
                    })
                );
                subscription.voiceConnection.on("error", console.warn);
                int.client.subscription.set(int.guildId!, subscription);
            }
        }


        if (!subscription) {
            return "Join a voice channel and then try that again!";
        }

        try {
            await waitForResourceToEnterState(subscription.voiceConnection, VoiceConnectionStatus.Ready, 20000);
        } catch (error) {
            console.warn(error);
            return "Failed to join voice channel within 20 seconds, please try again later!";
        }

        let loop = false;
        try {
            if (song instanceof Array) {
                let track: Track;
                song.forEach(async s => {
                track = await Track.from(s, int.user, int.channel, int.guildId!, loop, {
                    onStart () {
                        reply(int, { content: `Now playing: ${s}` });
                    },
                    onError (error) {
                        console.warn(error);
                        reply(int, { content: `Error: ${error.message}....`, ephemeral: true });
                    },
                });
                subscription!.enqueue(track);
            })
                reply(int, `Queued ${url} playlist.`);
            } else {
            const track = await Track.from(song, int.user, int.channel, int.guildId!, loop, {
                onStart () {
                    reply(int, { content: "Now playing!" });
                },
                onError (error) {
                    console.warn(error);
                    reply(int, { content: `Error: ${error.message}...`, ephemeral: true });
                },
            });
            subscription.enqueue(track);
            reply(int, `Queued **${track.title}**`);
        }
        } catch (error) {
            console.warn(error);
            reply(int, "Failed to play track, please try again later!");
        }
    },
});
