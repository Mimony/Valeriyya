import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { AppOptionTypes, defineCommand, type IContextInteraction } from "../../lib/util/valeriyya.types";
import type { GuildMember } from "discord.js";
import { getUserHistory } from "../../lib/util/moderation/valeriyya.moderation";

export default defineCommand({
    data: {
        name: "history",
        type: AppOptionTypes.USER,
    },
    menu: async (int: IContextInteraction) => {
        const target = int.options.getMember("user") as GuildMember;
        const db = int.client.settings
        const history = await getUserHistory({ id: target.id, db, gid: int.guildId! });

        if (!history) return {
            embeds: [
                new ValeriyyaEmbed()
                    .setDescription("There is no past moderation actions for this user.")
            ]
        }

        return {
            embeds: [
                new ValeriyyaEmbed()
                    .setAuthor({ name: `${target.user.tag}'s moderation history:` })
                    .setDescription(`\`\`\`bans: ${history.ban}\nkicks: ${history.kick}\nmutes: ${history.mute}\`\`\``)
            ]
        }

    }
})