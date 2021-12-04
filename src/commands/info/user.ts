import { GuildMember } from "discord.js";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

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

    const embed = new ValeriyyaEmbed()
    .setAuthor(`${user.tag} (${user.id})`, user.displayAvatarURL({ dynamic: true }))
    .setDescription(`Nickname: ${member.nickname ?? "None"}`)

    return {
      embeds: [embed],
    };
  },
});
