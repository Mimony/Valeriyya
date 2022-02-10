import type { GuildMember } from "discord.js";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

export default defineCommand({
  data: {
    name: "resume",
    description: "Resume the current song.",
  },
  chat: async (int: ICommandInteraction) => {
    const subscription = int.client.subscription.get(int.guildId!);
    const member = int.member as GuildMember;

    if (subscription) {
      if (member.voice.channelId !== int.guild!.me?.voice.channelId)
        return {
          content: "You must be in the same voice channel as me to use this command! <3",
          ephemeral: true,
        };

      subscription.audioPlayer.unpause();

      return `${int.user} has resumed the song`;
    } else {
        return "There is no music currently playing."
    }
  }
});
