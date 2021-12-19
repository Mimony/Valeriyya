import { type GuildMember, Message, MessageActionRow, MessageSelectMenu } from "discord.js";
import { defineCommand, type ICommandInteraction, OptionTypes } from "../../lib/util/valeriyya.types";
import { ValeriyyaEmbed } from "../../lib/util/valeriyya.embed";

export default defineCommand({
    data: {
        name: "settings",
        description: "Changes the settings in this guild.",
        options: [
            {
                name: "type",
                description: "The settings type to change.",
                type: OptionTypes.STRING,
                required: true,
                choices: [
                    {
                        name: "channels",
                        value: "channels"
                    },
                    {
                        name: "roles",
                        value: "roles"
                    }
                ]
            },
        ]
    },
    chat: async (int: ICommandInteraction) => {
        await int.deferReply({ ephemeral: true });
        const member = int.member as GuildMember;
        const choice = int.options.getString("type", true);
        const db = await int.client.db(int.guild!);


        if (!member.permissions.has("MANAGE_GUILD", true)) return {
            embeds: [
                new ValeriyyaEmbed(undefined, "error")
                    .setAuthor(`${int.user.tag} (${int.user.id})`, int.user.displayAvatarURL({ dynamic: true }))
                    .setDescription("You are missing the `MANAGE_GUILD` permission")
            ]
        }
        let row = new MessageActionRow()

        if (choice === "roles") {
            let roleMenu = new MessageSelectMenu()
                .setCustomId("settings.roles")
                .setPlaceholder("Provide a role for the selected type above.")
            await int.guild!.roles.cache.filter(r => r.guild.id !== r.id).each(r => {
                roleMenu.addOptions({
                    label: r.name,
                    description: "Select this role to set it as a staff role.",
                    value: r.id
                })
            })
            row.setComponents(roleMenu)
        } else if (choice === "channels") {
            let channelMenu = new MessageSelectMenu()
                .setCustomId("settings.channels")
                .setPlaceholder("Provide a channel for the selected type above.")
            await int.guild!.channels.cache.filter(c => c.type === "GUILD_TEXT").each(r => {
                channelMenu.addOptions({
                    label: r.name,
                    description: "Select this channel to set it as a log channel.",
                    value: r.id
                })
            })
            row.setComponents(channelMenu)
        }

        const menu = await int.followUp({
            content: `Select the ${choice} setting below.`,
            components: [row],
        });

        const collector = await (menu as Message).awaitMessageComponent({ time: 15000, componentType: "SELECT_MENU" })
        if (collector.customId === "settings.roles") {
            db.roles.staff = collector.values[0];
            db.save();
            collector.reply({ content: `The staff role has been updated to ${int.guild!.roles.resolve(collector.values[0])}`, ephemeral: true })
        } else if (collector.customId === "settings.channels") {
            db.channels.logs = collector.values[0];
            db.save();
            collector.reply({ content: `The logs channel has been updated to ${int.guild!.channels.resolve(collector.values[0])}`, ephemeral: true })
        }
        return;
    }
})