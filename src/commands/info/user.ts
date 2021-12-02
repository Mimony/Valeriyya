import { GuildMember, MessageEmbed } from "discord.js";
import { defineCommand, ICommandInteraction } from "../../lib/util/utilityTypes";

export default defineCommand({
  name: "user",
  description: "Gets the information about a user.",
  data: [
    {
      name: "user",
      description: "Gets the information about this user.",
      type: "USER",
    },
  ],
  execute: (int: ICommandInteraction) => {
    const member = int.options.getMember("user") || int.member;

    if (!(member instanceof GuildMember)) return;
    const { user } = member!;

    const embed = new MessageEmbed()
    .setAuthor(`${user.tag} (${user.id})`, user.displayAvatarURL({ dynamic: true }))
    .setDescription(`Nickname: ${member.nickname ?? "None"}`)

    return {
      embeds: [embed],
    };
  },
});
