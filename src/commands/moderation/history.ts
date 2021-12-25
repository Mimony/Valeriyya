import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { AppOptionTypes, defineCommand, type IContextInteraction } from "../../lib/util/valeriyya.types";
import type { GuildMember } from "discord.js";

export default defineCommand({
    data: {
        name: "history",
        type: AppOptionTypes.USER,
    },
    menu: async (int: IContextInteraction) => {
        const target = int.options.getMember("user") as GuildMember;
        const db = await int.client.db(int.guild!);
        const history = (await db.getUserHistory(target.id))!;

        return {
            embeds: [
                new ValeriyyaEmbed()
                    .setAuthor(`${target.user.tag}'s moderation history:`)
                    .setDescription(`\`\`\`bans: ${history.ban}\nkicks: ${history.kick}\nmutes: ${history.mute}\`\`\``)
            ]
        }

    }
})