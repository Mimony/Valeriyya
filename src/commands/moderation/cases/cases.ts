import { defineCommand, type ICommandInteraction, OptionTypes } from "../../../lib/util/valeriyya.types";
import { GuildMember } from "discord.js";
import { ValeriyyaEmbed } from "../../../lib/util/valeriyya.embed";

export default defineCommand({
    data: {
        name: "case",
        description: "Gets|deletes cases.",
        options: [
            {
                name: "option",
                description: "What to do with the case.",
                type: OptionTypes.STRING,
                choices: [
                    {
                        name: "show",
                        value: "show"
                    },
                    {
                        name: "delete",
                        value: "delete"
                    },
                ],
                required: true
            },
            {
                name: "id",
                description: "The id of the case.",
                type: OptionTypes.NUMBER,
                required: true
            }
        ]
    },
    execute: async (int: ICommandInteraction) => {
        const member = int.member;
        const db = await int.client.db(int.guild!);
        const id = int.options.getNumber("id")!;
        const options = int.options.getString("option")!;

        if (!(member instanceof GuildMember)) return;

        if (!member.permissions.has("MANAGE_GUILD", true)) {
            const embed = new ValeriyyaEmbed(undefined, "error")
                .setAuthor(`${int.user.tag} (${int.user.id})`, int.user.displayAvatarURL({dynamic: true}))
                .setDescription("You are missing the `MANAGE_GUILD` permission");

            return {
                embeds: [embed],
                ephemeral: true
            }
        }

        if (options === "show") {
            const c = db.getCaseById(id);
            if (!c) return {
                content: `There is no such case with the id ${id}`,
                ephemeral: true,
            }

            const staff = await int.guild!.members.fetch(c.staffId);
            const target = await int.client.users.fetch(c.targetId);

            const embed = await int.client.cases.log({
                action: c.action,
                staff,
                target,
                id: c.id,
                reason: c.reason,
            })

            return {
                embeds: [embed],
                ephemeral: true
            }
        }
        else if (options === "delete") {
            const c = db.getCaseById(id);
            if (!c) return {
                content: `There is no such case with the id ${id}`,
                ephemeral: true,
            }

            await db.removeCase(id)

            const embed = new ValeriyyaEmbed()
                .setDescription(`Successfully removed case with the id ${id}`)

            return {
                embeds: [embed],
                ephemeral: true
            }
        }
        return;
    }
})