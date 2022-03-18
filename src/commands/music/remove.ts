import type { GuildMember } from "discord.js";
import { defineCommand, ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "remove",
        description: "Remove a song from the queue.",
        options: [
            {
                name: "id",
                description: "The id of the song",
                type: OptionTypes.NUMBER,
            }
        ]
    },
    chat: async (int: ICommandInteraction) => {
        const subscription = int.client.subscription.get(int.guildId!);
        const member = int.member as GuildMember;
        const songID = int.options.getNumber("id")!;

        if (subscription) {
            if (member.voice.channelId !== int.guild!.me?.voice.channelId) return {
                content: "You must be in the same voice channel as me to use this command! <3",
                ephemeral: true
            }

            if (songID! < 1 || songID! > subscription!.queue.length){
                return await int!.reply({ content: `you never provided a valid song ID! A valid ID is a number between 1 - ${subscription!.queue.length}`, ephemeral: true})
            }

            let toRemoveInArray = subscription!.queue[songID - 1];
            let toRemove = `[${toRemoveInArray.title}](<${toRemoveInArray.url}>)`;
            subscription!.queue.splice(songID - 1, 1);

            return `${toRemove} has been removed from the queue`
        } else {
            return "There is no music currently playing."
        }

    }
})