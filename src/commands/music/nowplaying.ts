import { AudioPlayerStatus, AudioResource } from "@discordjs/voice";
import type { GuildMember } from "discord.js";
import type { Track } from "../../lib/util/valeriyya.music";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "nowplaying",
        description: "Show the now playing song.",
    },
    chat: async (int: ICommandInteraction) => {
        const subscription = int.client.subscription.get(int.guildId!);
        const member = int.member as GuildMember;

        if (subscription) {
            if (member.voice.channelId !== int.guild!.me?.voice.channelId) return {
                content: "You must be in the same voice channel as me to use this command! <3",
                ephemeral: true
            }

            const currentPlaying = subscription.audioPlayer.state.status === AudioPlayerStatus.Idle ?
            `There is no song currently playing!`:
            `Currently playing: **[${(subscription?.audioPlayer.state.resource as AudioResource<Track>).metadata.title}](<${(subscription?.audioPlayer.state.resource as AudioResource<Track>).metadata.url}>)**`;
            
            return currentPlaying;
        } else {
            return "There is no music currently playing."
        }

    }
})