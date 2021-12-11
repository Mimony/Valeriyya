import { GuildMember } from "discord.js";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, ICommandInteraction } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "user",
        description: "Gets the information about a user.",
        options: [
            {
                name: "user",
                description: "Gets the information about this user.",
                type: "USER",
            },
        ],
    },
    execute: (int: ICommandInteraction) => {
        const member = int.options.getMember("user") || int.member;

        if (!(member instanceof GuildMember)) return;
        const {user} = member!;

        const embed = new ValeriyyaEmbed()
            .setAuthor(`${user.tag} (${user.id})`, user.displayAvatarURL({dynamic: true}))
            .setDescription(`
    User Created at: ${timeFormat(user.createdAt)} ${user.bot ? "(User is a bot)" : ""}
    Member Joined At: ${timeFormat(member.joinedAt)}
    Roles: ${member.roles.cache.filter(r => member.guild.roles.everyone !== r).map(r => r).slice(0, 10)}
    Moderation History: __SOON__
    `)

        return {
            embeds: [embed],
        };
    },
});

function timeFormat(date: Date | null): string {
    const time = Math.floor(date!.getTime());
    return `<t:${time}:d>`;
}