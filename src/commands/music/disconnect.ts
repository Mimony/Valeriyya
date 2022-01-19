import type { GuildMember } from "discord.js";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

export default defineCommand({
  data: {
    name: "disconnect",
    description: "Disconnect from the current channel.",
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

      subscription.voiceConnection.destroy();
      int.client.subscription.delete(int.guildId!);

      return `${int.user} has disconnected the bot.`;
    } else {
        return "I'm not playing any music in this server."
    }
  }
});
