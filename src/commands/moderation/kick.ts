import { GuildMember } from "discord.js";
import { Kick } from "../../lib/util/moderation/valeriyya.moderation.kick";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, type ICommandInteraction } from "../../lib/util/valeriyya.types";
import { ApplicationCommandOptionTypes } from "discord.js/typings/enums";

export default defineCommand({
    data: {
        name: "kick",
        description: "Kicks a member from the guild.",
        options: [
            {
                name: "member",
                description: "The member to kick.",
                type: ApplicationCommandOptionTypes.USER,
                required: true
            }
        ]
    },
    execute: async (int: ICommandInteraction) => {
        const staff = int.member;
        const target = int.options.getMember("member");

        if (!(staff instanceof GuildMember) || !(target instanceof GuildMember)) return;

        const date = new Date();
        const action = new Kick({
            int,
            staff,
            target,
            date
        });

        await action.all();

        const embed = new ValeriyyaEmbed()
            .setAuthor(`${int.user.tag} (${int.user.id})`, int.user.displayAvatarURL({dynamic: true}))
            .setThumbnail(int.guild?.iconURL({dynamic: true}) ?? '')
            .setDescription(`${target} has been kicked from ${int.guild?.name}`);

        return {
            embeds: [embed]
        }
    }
})