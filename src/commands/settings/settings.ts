import { GuildMember, Message, MessageActionRow, MessageSelectMenu } from "discord.js";
import { defineCommand, type ICommandInteraction } from "../../lib/util/valeriyya.types";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";

export default defineCommand({
    data: {
        name: "settings",
        description: "Changes the settings in this guild."
    },
    execute: async (int: ICommandInteraction) => {
        await int.deferReply({ ephemeral: true });
        const member = int.member;
        const db = await int.client.db(int.guild!);

        if (!(member instanceof GuildMember)) return;

        if (!member.permissions.has("MANAGE_GUILD", true)) return {
            embeds: [
                new ValeriyyaEmbed(undefined, "error")
                    .setAuthor(`${int.user.tag} (${int.user.id})`, int.user.displayAvatarURL({ dynamic: true }))
                    .setDescription("You are missing the `MANAGE_GUILD` permission")
            ]
        }
        let row = new MessageActionRow()

            let settingsMenu = new MessageSelectMenu()
                .setCustomId("settings")
                .setPlaceholder("Select what setting to change.")
                .addOptions(
                    {
                        value: "staff",
                        description: "Choose a staff role.",
                        label: "staff"
                    },
                    {
                        value: "mute",
                        description: "Choose a mute role.",
                        label: "mute"
                    },
                    {
                        value: "welcome",
                        description: "Choose a welcome channel.",
                        label: "welcome"
                    },
                    {
                        value: "logs",
                        description: "Choose a logs channel.",
                        label: "logs"
                    }
                )
            row.setComponents(settingsMenu)

        let channelsMenu1 = new MessageSelectMenu()
            .setCustomId("settings.channels.welcome")
            .setPlaceholder("Provide a channel that will be used for welcome messages.")
        int.guild!.channels.cache.filter(c => {
            if (member.permissions.has("ADMINISTRATOR")) return c.type === "GUILD_TEXT";
            return c.type === "GUILD_TEXT" && c.permissionsFor(member, true).has("VIEW_CHANNEL");
        }).each(c => {
            // TODO Divide Channels in more menus
            channelsMenu1.addOptions(
                {
                    value: c.id,
                    description: `Select a channel to set it as a welcome channel.`,
                    label: c.name
                }
            )
        })
        let channelsMenu2 = new MessageSelectMenu()
            .setCustomId("settings.channels.logs")
            .setPlaceholder("Provide a channel that will be used for logs.")

        int.guild!.channels.cache.filter(c => {
            if (member.permissions.has("ADMINISTRATOR")) return c.type === "GUILD_TEXT";
            return c.type === "GUILD_TEXT" && c.permissionsFor(member, true).has("VIEW_CHANNEL");
        }).each(c => {
            // TODO Divide Channels in more menus
            channelsMenu2.addOptions(
                {
                    value: c.id,
                    description: `Select a channel to set it as a logs channel.`,
                    label: c.name
                }
            )
        })

        let rolesMenu1 = new MessageSelectMenu()
            .setCustomId("settings.roles.staff")
            .setPlaceholder("Provide a role that will be used as a staff role.")

        int.guild!.roles.cache.filter(r => r.id !== r.guild.id).each(r => {
            // TODO Divide Roles in more menus
            rolesMenu1.addOptions(
                {
                    value: r.id,
                    description: `Select a role to set it as a staff role.`,
                    label: r.name
                }
            )
        })


        let rolesMenu2 = new MessageSelectMenu()
            .setCustomId("settings.roles.mute")
            .setPlaceholder("Provide a role that will be used as a mute role.")

        int.guild!.roles.cache.filter(r => r.id !== r.guild.id).each(r => {
            // TODO Divide Roles in more menus
            rolesMenu2.addOptions(
                {
                    value: r.id,
                    description: `Select a role to set it as a mute role.`,
                    label: r.name
                }
            )
        })

        const settings_menu = await int.followUp({
            content: `Select what role/channel to change.`,
            components: [row],
            fetchReply: true
        });

        let menu;

        try {
            const type_collector = await (settings_menu as Message).awaitMessageComponent({
                time: 30000,
                componentType: "SELECT_MENU"
            });

            if (type_collector.values[0] === "welcome") {
                menu = await type_collector.update({
                    content: `Select a ${type_collector.values[0]} channel`,
                    components: [row.setComponents(channelsMenu1)],
                    fetchReply: true
                })
            } else if (type_collector.values[0] === "logs") {
                menu = await type_collector.update({
                    content: `Select a ${type_collector.values[0]} channel`,
                    components: [row.setComponents(channelsMenu2)],
                    fetchReply: true
                })
            }
            if (type_collector.values[0] === "staff") {
                menu = await type_collector.update({
                    content: `Select a ${type_collector.values[0]} role.`,
                    components: [row.setComponents(rolesMenu1)],
                    fetchReply: true
                })
            } else if (type_collector.values[0] === "mute") {
                menu = await type_collector.update({
                    content: `Select a ${type_collector.values[0]} role.`,
                    components: [row.setComponents(rolesMenu2)],
                    fetchReply: true
                })
            }
        } catch (e: any) {
            int.client.logger.error`The type collector has failed ${e.message}`
            int.editReply({
                content: `Selection has been canceled.`,
                components: []
            })
        }

        try {
            const collector = await (menu as Message).awaitMessageComponent({
                time: 30000,
                componentType: "SELECT_MENU"
            })

            if (collector.customId === "settings.roles.staff") {
                db.roles.staff = collector.values[0];
                db.save();
                collector.update({
                    content: `The staff role has been updated to ${int.guild!.roles.resolve(collector.values[0])}`,
                    components: []
                })
            } else if (collector.customId === "settings.channels.logs") {
                db.channels.logs = collector.values[0];
                db.save();
                collector.update({
                    content: `The logs channel has been updated to ${int.guild!.channels.resolve(collector.values[0])}`,
                    components: []
                })
            } else if (collector.customId === "settings.roles.mute") {
                db.roles.mute = collector.values[0];
                db.save();
                collector.update({
                    content: `The mute role has been updated to ${int.guild!.roles.resolve(collector.values[0])}`,
                    components: []
                })
            } else if (collector.customId === "settings.channels.welcome") {
                db.channels.welcome = collector.values[0];
                db.save();
                collector.update({
                    content: `The welcome channel has been updated to ${int.guild!.channels.resolve(collector.values[0])}`,
                    components: []
                })
            }
        } catch (e: any) {
            int.client.logger.error`The role | channel collector has failed ${e.message}`
            int.editReply({
                content: `Selection has been canceled.`,
                components: []
            })
        }
        return;
    }
})