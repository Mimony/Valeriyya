import type { GuildMember } from "discord.js";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";

export default defineCommand({
    data: {
        name: "user",
        description: "Gets the information about a user.",
        options: [
            {
                name: "user",
                description: "Gets the information about this user.",
                type: OptionTypes.USER,
            },
        ],
    },
    chat: async (int: ICommandInteraction) => {
        const member = (int.options.getMember("user") || int.member) as GuildMember;
        const db = await int.client.db(int.guild!);
        const history = (await db.getUserHistory(member.id))!
        const { user } = member!;

        const embed = new ValeriyyaEmbed()
            .setAuthor(`${user.username}`, user.displayAvatarURL({dynamic: true}))
            .setDescription(`
    User Created at: ${timeFormat(user.createdAt)} ${user.bot ? "(User is a bot)" : ""}
    Member Joined At: ${timeFormat(member.joinedAt)}
    ID: ${user.id}
    Moderation History: \`\`\`bans: ${history.ban}\nkicks: ${history.kick}\nmutes: ${history.mute}\`\`\`
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