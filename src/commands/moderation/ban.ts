import { GuildMember } from "discord.js";
import { Ban } from "../../lib/util/moderation/valeriyya.moderation.ban";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "ban",
        description: "Bans a member from the guild.",
        options: [
            {
                name: "member",
                description: "The member to ban.",
                type: OptionTypes.USER,
            },
            {
                name: "member-id",
                description: "The member to ban. (Use this to provide an id instead of mention)",
                type: OptionTypes.STRING,
            },
            {
                name: "reason",
                description: "The reason for this ban.",
                type: OptionTypes.STRING
            }
        ]
    },
    execute: async (int: ICommandInteraction) => {
        const staff = int.member;
        const target_options = int.options.getMember("member") || int.options.getString("member-id");
        const reason = int.options.getString("reason") ?? "";
        let target;

        if (!(staff instanceof GuildMember)) return;

        if (target_options instanceof String) {
            target = await int.client.users.fetch(int.options.getString("member-id")!)
        } else {
            target = int.options.getMember("member")! as GuildMember;
        }

        if (int.guild?.bans.cache.has(target.id)) {
            return {
                ephemeral: true,
                content: `This member is already banned from this guild.`
            }
        }

        const date = Date.now();
        const action = new Ban({
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
            .setDescription(`${target} has been banned from ${int.guild?.name}`);

        return {
            embeds: [embed],
            ephemeral: true
        }
    }
})