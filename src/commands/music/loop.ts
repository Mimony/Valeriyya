import type { GuildMember } from "discord.js";
import { defineCommand, ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";

export default defineCommand({
  data: {
    name: "loop",
    description: "Loop thru a song or the entire queue.",
    options: [
      {
        name: "type",
        description: "Choose what will be looped.",
        type: OptionTypes.STRING,
        required: true,
        choices: [
          {
            name: "song",
            value: "song",
          },
          {
            name: "queue",
            value: "queue",
          },
        ],
      },
    ],
  },
  chat: async (int: ICommandInteraction) => {
    const subscription = int.client.subscription.get(int.guildId!);
    const member = int.member as GuildMember;
    const type = int.options.getString("type")!;

    if (!subscription) return "There is no music currently playing.";
    if (member.voice.channelId !== int.guild!.me?.voice.channelId)
      return {
        content: "You must be in the same voice channel as me to use this command! <3",
        ephemeral: true,
      };

    if (type === "song") {
      if (subscription.queueLoop) return `You can't loop thru a song when the queue is looping. Plesae first disable the queue loop to enable song loop.`;

      if (subscription.currentPlaying?.looping) {
        subscription.currentPlaying!.looping = false;

        return `Song Loop has been **Disabled**.`;
      } else {
        subscription.currentPlaying!.looping = true;

        return `Song loop has been **Enabled**.`;
      }
    } else {
      if (subscription.currentPlaying?.looping)
        return `You can't loop thru a queue when a song is looping. Please first disable the song loop to enable queue loop.`;

      if (subscription.queueLoop) {
        subscription.queueLoop = false;

        return `Queue Loop has been **Disabled**.`;
      } else {
        subscription.queueLoop = true;

        return `Queue Loop has been **Enabled**.`;
      }
    }


  },
});
