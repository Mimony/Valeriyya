import type { GuildMember } from "discord.js";
import { Kick } from "../../lib/util/moderation/valeriyya.moderation.kick";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "kick",
        description: "Kicks a member from the guild.",
        options: [
            {
                name: "member",
                description: "The member to kick.",
                type: OptionTypes.USER,
                required: true
            },
            {
                name: "reason",
                description: "The reason for this ban.",
                type: OptionTypes.STRING
            }
        ]
    },
    chat: async (int: ICommandInteraction) => {
        const staff = int.member as GuildMember;
        const target = int.options.getMember("member") as GuildMember;
        const reason = int.options.getString("reason") ?? "";


        const date = Date.now();
        const action = new Kick({
            int,
            staff,
            target,
            date,
            reason
        });

        await action.all();

        const embed = new ValeriyyaEmbed()
            .setAuthor(`${int.user.tag} (${int.user.id})`, int.user.displayAvatarURL({dynamic: true}))
            .setThumbnail(int.guild?.iconURL({dynamic: true}) ?? '')
            .setDescription(`${target} has been kicked from ${int.guild?.name}`);

        return {
            embeds: [embed],
            ephemeral: true
        }
    }
})