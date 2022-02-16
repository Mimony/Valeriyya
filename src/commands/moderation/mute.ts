import type { GuildMember } from "discord.js";
import { Mute } from "../../lib/util/moderation/valeriyya.moderation.mute";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";
import ms from "../../lib/util/valeriyya.ms";

export default defineCommand({
    data: {
        name: "mute",
        description: "Mutes a member for a specified time.",
        options: [
            {
                name: "member",
                description: "The member to ban.",
                type: OptionTypes.USER,
                required: true
            },
            {
                name: "time",
                description: "The time the member to be muted for. (Max 28 days).",
                type: OptionTypes.STRING,
                required: true
            },
            {
                name: "reason",
                description: "The reason for this ban.",
                type: OptionTypes.STRING,
                required: false
            }
        ]
    },
    chat: async (int: ICommandInteraction) => {
        const staff = int.member as GuildMember;
        const target = int.options.getMember("member") as GuildMember;
        const reason = int.options.getString("reason") ?? "";
        const time = int.options.getString("time")!;

        if (!target.moderatable) return {
            embeds: [
                new ValeriyyaEmbed(undefined, "error")
                .setDescription("I can't mute this person due to me being unable to manage him. (They have a higher role than me or they are the owner).",)
                .setAuthor({ name: `${int.user.tag} (${int.user.id})`, url: int.user.displayAvatarURL({ dynamic: true }) })
            ],
            ephemeral: true
        }

        if (!time.match(/(?<amount>\d+)\s?(?<unit>s|sec|second|seconds|m|min|minute|minutes|h|hour|hours|d|day|days|w|week|weeks)/)) return {
            embeds: [new ValeriyyaEmbed(undefined, "error")
            .setDescription(`Please provide the right syntax for the time paramater!
            Example: 1m, 2d, 3w (1 min, 2 days, 3 weeks)`)
            .setAuthor({ name: `${int.user.tag} (${int.user.id})`, url: int.user.displayAvatarURL({ dynamic: true }) })
            ],
            ephemeral: true,
        }

        const duration = ms(time);

        if (duration >= 2.419e+9) return {
            embeds: [new ValeriyyaEmbed(undefined, "error")
            .setDescription("The amount of duration you can assign a mute is 28 days!")
            .setAuthor({ name: `${int.user.tag} (${int.user.id})`, url: int.user.displayAvatarURL({ dynamic: true }) })
            ],
            ephemeral: true,
        }

        const date = Date.now();
        const action = new Mute({
            int,
            staff,
            target,
            date,
            reason,
            duration
        });

        await action.all();

        const embed = new ValeriyyaEmbed()
            .setAuthor({ name: `${int.user.tag} (${int.user.id})`, iconURL: int.user.displayAvatarURL({ dynamic: true }) })
            .setThumbnail(int.guild?.iconURL({dynamic: true}) ?? '')
            .setDescription(`${target} has been muted from ${int.guild?.name} for ${time}`);

        return {
            embeds: [embed],
            ephemeral: true
        }
    }
})