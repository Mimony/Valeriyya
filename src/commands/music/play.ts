import { DiscordGatewayAdapterCreator, joinVoiceChannel, VoiceConnectionStatus } from "@discordjs/voice";
import { GuildMember } from "discord.js";
import { MusicSubscription, Track } from "../../lib/util/valeriyya.music";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";
import { reply } from "../../lib/util/valeriyya.util";
import { waitForResourceToEnterState } from "../../lib/util/valeriyya.music";
import play from "play-dl"
import { validateURL } from "ytdl-core";

export default defineCommand({
    data: {
        name: "play",
        description: "Play a song thru the bot.",
        options: [
            {
                name: "song",
                description: "The song that will be played.",
                type: OptionTypes.STRING,
                required: true
            }
        ]
    },
    // @ts-ignore
    chat: async (int: ICommandInteraction) => {
        let subscription = int.client.subscription.get(int.guildId!);
        const url = int.options.getString("song")!;
        let song: string;

        const validate = validateURL(url);
        // @ts-ignore
        // @DecrepitHuman this needs your attention
        const videos = await (await play.playlist_info(url)).all_videos();
        
        await int.deferReply();
        
        if (!subscription) {
            if (int.member instanceof GuildMember && int.member.voice.channel){
                const channel = int.member.voice.channel;
                subscription = new MusicSubscription(
                    joinVoiceChannel({
                        channelId: channel.id,
                        guildId: channel.guild.id,
                        adapterCreator: int.guild!.voiceAdapterCreator as DiscordGatewayAdapterCreator,
                    }),
                    );
                    subscription.voiceConnection.on('error', console.warn);
                    int.client.subscription.set(int.guildId!, subscription);
                }
            }
    
            if (url.match("^(?:spotify:|https:\/\/[a-z]+\.spotify\.com\/(track\/|user\/(.*)\/playlist\/))(.*)$")) {
                if (play.is_expired()) await play.refreshToken();
    
                const spotify = await play.spotify(url);
                const searched = await play.search(spotify.name, { limit: 1 });
                song = searched[0].url
            } else if (!validate) {
                let videos = await play.search(url, { limit: 1 });
                if (videos.length === 0) return {
                    content: "I couldn't find the song you were looking for. Please try to search a more specific name."
                };
                song = videos[0].url;
            } else song = url;

        if (!subscription) {
            return 'Join a voice channel and then try that again!';
		}

        try {
			await waitForResourceToEnterState(subscription.voiceConnection, VoiceConnectionStatus.Ready, 20000);
		} catch (error) {
			console.warn(error);
			return 'Failed to join voice channel within 20 seconds, please try again later!'
		}

        let loop = false;
        try {
			const track = await Track.from(song, int.user, int.channel, int.guildId!, loop, {
				onStart() {
					reply(int, { content: 'Now playing!' })
				},
				onFinish() {
					reply(int, { content: 'Now finished!' })
				},
				onError(error) {
					console.warn(error);    
					reply(int, { content: `Error: ${error.message}`, ephemeral: true})
				},
			});
			subscription.enqueue(track);
			reply(int, `Queued **${track.title}**`);
		} catch (error) {
			console.warn(error);
			reply(int, 'Failed to play track, please try again later!');
		}
    }
})