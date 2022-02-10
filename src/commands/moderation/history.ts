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
        const db = await int.client.guild.get(int.guildId!);
        const history = getUserHistory({ id: target.id, db, client: int.client })!;

        return {
            embeds: [
                new ValeriyyaEmbed()
                    .setAuthor({ name: `${target.user.tag}'s moderation history:` })
                    .setDescription(`\`\`\`bans: ${history.ban}\nkicks: ${history.kick}\nmutes: ${history.mute}\`\`\``)
            ]
        }

    }
})