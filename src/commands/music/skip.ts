import type { GuildMember } from "discord.js";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

export default defineCommand({
  data: {
    name: "skip",
    description: "Skip the current playing song.",
  },
  chat: async (int: ICommandInteraction) => {
    const subscription = int.client.subscription.get(int.guildId!);
    const member = int.member as GuildMember;

    if (!subscription) return "There is no music currently playing.";
    if (member.voice.channelId !== int.guild!.me?.voice.channelId)
      return {
        content: "You must be in the same voice channel as me to use this command! <3",
        ephemeral: true,
      };

    subscription.audioPlayer.stop();

    return `${int.user} has skipped the song`;
  },
});
