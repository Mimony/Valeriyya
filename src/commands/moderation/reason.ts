import { GuildMember } from "discord.js";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { getCaseById } from "../../lib/util/moderation/valeriyya.moderation";

export default defineCommand({
    data: {
        name: "reason",
        description: "Changes the reason for a case.",
        options: [
            {
                name: "id",
                description: "The case id to change the case reason for.",
                type: OptionTypes.NUMBER,
                required: true
            },
            {
                name: "reason",
                description: "The reason for this ban.",
                type: OptionTypes.STRING,
                required: true
            }
        ]
    },
    chat: async (int: ICommandInteraction) => {
        const member = int.member;
        const id = int.options.getNumber("id")!;
        const reason = int.options.getString("reason")!;
        const db = int.client.settings

        if (!(member instanceof GuildMember)) return;

        if (!member.permissions.has("MANAGE_GUILD", true)) return {
            embeds: [
                new ValeriyyaEmbed(undefined, "error")
                .setAuthor({ name: `${int.user.tag} (${int.user.id})`, url: int.user.displayAvatarURL({ dynamic: true }) })
                .setDescription("You are missing the `MANAGE_GUILD` permission")
            ]
        }

        const c = getCaseById({ gid: int.guildId!, id, db, client: int.client });
        if (!c) return { context: c, ephemeral: true }

        await int.client.cases.edit({
            guildId: int.guild!.id,
            id,
            reason
        })

        return {
            embeds: [
                new ValeriyyaEmbed()
                    .setDescription(`The cases reason with an id ${id} has been successfully changed.`)
            ],
            ephemeral: true
        }
    }
})