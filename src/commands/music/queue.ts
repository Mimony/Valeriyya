import type { GuildMember } from "discord.js";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "queue",
        description: "Show queue of songs.",
    },
    chat: async (int: ICommandInteraction) => {
        const subscription = int.client.subscription.get(int.guildId!);
        const member = int.member as GuildMember;

        if (subscription) {
            if (member.voice.channelId !== int.guild!.me?.voice.channelId) return {
                content: "You must be in the same voice channel as me to use this command! <3",
                ephemeral: true
            }
        
            
            return subscription.queue
				.slice(0, 5)
				.map((track, index) => `${index + 1}) [${track.title}](${track.url})`)
				.join('\n');

        } else {
            return "There is no music currently playing."
        }

    }
})