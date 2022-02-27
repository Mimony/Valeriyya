import type { GuildMember, Role, TextBasedChannel } from "discord.js";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";

export default defineCommand({
    data: {
        name: "settings",
        description: "Changes the settings in this guild.",
        options: [
            {
                name: "channel",
                description: "Select a channel setting.",
                type: OptionTypes.SUB_COMMAND,
                options: [
                    {
                        name: "type",
                        description: "Choose what the channel is for.",
                        type: OptionTypes.STRING,
                        choices: [
                            {
                                name: "logs",
                                value: "logs"
                            },
                            {
                                name: "welcome",
                                value: "welcome"
                            }
                        ],
                        required: true,
                    },
                    {
                        name: "channel",
                        description: "The channel that will be used for the previous type.",
                        type: OptionTypes.CHANNEL,
                        channelTypes: ["GUILD_TEXT", "GUILD_NEWS"],
                        required: true
                    }
                ]
            },
            {
                name: "role",
                description: "Select a role setting.",
                type: OptionTypes.SUB_COMMAND,
                options: [
                    {
                        name: "type",
                        description: "Choose what the role is for.",
                        type: OptionTypes.STRING,
                        choices: [
                            {
                                name: "staff",
                                value: "staff"
                            },
                            {
                                name: "mute",
                                value: "mute"
                            }
                        ],
                        required: true,
                    },
                    {
                        name: "role",
                        description: "The role that will be used for the previous type.",
                        type: OptionTypes.ROLE,
                        required: true
                    }
                ]
            }
        ]
    },
    chat: async (int: ICommandInteraction) => {
        const member = int.member as GuildMember;
        const db = int.client.settings
        const cmd = int.options.getSubcommand();
        const channel_type = int.options.getString("type") as "channel.mod" | "channel.welcome";
        const role_type = int.options.getString("type") as "role.mod" | "role.mute";
        const role = int.options.getRole("role") as Role;
        const channel = int.options.getChannel("channel") as Omit<TextBasedChannel, "DMChannel" | "PartialDMChannel" | "ThreadChannel">;

        if (!member.permissions.has("MANAGE_GUILD", true)) return {
            embeds: [
                new ValeriyyaEmbed(undefined, "error")
                    .setAuthor({ name: `${int.user.tag} (${int.user.id})`, url: int.user.displayAvatarURL({ dynamic: true }) })
                    .setDescription("You are missing the `MANAGE_GUILD` permission")
            ]
        }

        if (cmd === "channel") {
            db.set(int.guild!, channel_type, channel.id)

            return {
                content: `The ${channel_type} channel has been updated to ${channel}.`,
                ephemeral: true,
            }
        } else if (cmd === "role") {
            db.set(int.guild!, role_type, role.id)

            return {
                content: `The ${role_type} role has been updated to ${role}.`,
                ephemeral: true
            }
        }

        return "This is not ment to happen."
    }
})
