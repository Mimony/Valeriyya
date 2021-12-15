import { GuildMember, MessageEmbed, TextBasedChannels, User } from "discord.js";
import type { Valeriyya } from "../valeriyya.client";
import type { Case } from "./valeriyya.db.models";
import { ValeriyyaEmbed } from "./valeriyya.embed";

type CASE = Omit<Case, "id">;

export class ValeriyyaCases {
    public client: Valeriyya;

    public constructor(client: Valeriyya) {
        this.client = client;
    }


    public async add({ guildId, staffId, targetId, action, date, reason, duration }: CASE) {
        const guild = await this.client.guilds.fetch(guildId);
        const staff = await guild.members.fetch(staffId);
        const target = await this.client.users.fetch(targetId);

        const db = await this.client.db(guild);
        const id = db.cases_number;
        const channelId = db.channels?.logs;
        let message: string | undefined = undefined;

        try {
            if (!channelId) return;
            const channel = await guild.channels.fetch(channelId) as TextBasedChannels;

            message = (await channel.send({
                embeds: [await this.log({
                    action,
                    staff,
                    target,
                    id,
                    reason,
                    duration
                })]
            })).id;
        } catch (err: any) {
            this.client.logger.error(`There was an error logging the case: ${err}`)
        }


        let new_case: Case = {
            id,
            message,
            guildId,
            staffId,
            targetId,
            action,
            date,
            reason,
            duration
        };

        await db.addCase(new_case)

    }

    public async log({
                         action,
                         staff,
                         target,
                         id,
                         reason,
                         duration
                     }: { action: "ban" | "kick" | "mute" | "unban" | "unmute", staff: GuildMember, target: User, id: number, reason: string, duration?: number }): Promise<ValeriyyaEmbed> {
        return new ValeriyyaEmbed()
            .setAuthor(`${staff.user.tag} (${staff.user.id})`, staff.user.displayAvatarURL({ dynamic: true }))
            .setFooter(`Case: ${id}`)
            .setDescription(`Member: \`${target.tag}\`
            Action: \`${action}\`
            Reason: \`${reason}\`
            ${duration ? `Duration: \`${duration}\`` : ""}
            `);
    }

    public async edit({ guildId, id, reason }: { guildId: string, id: number, reason: string }) {
        const db = await this.client.db(guildId);
        const guild = await this.client.guilds.fetch(guildId);

        const c = db.getCaseById(id);
        if (!c) return `There is no such case with the id ${id}`;
        c.reason = reason;
        db.save();

        if (!c.message) return;
        const channel_id = db.channels.logs;
        try {
            const channel = await guild.channels.fetch(channel_id!) as TextBasedChannels;
            const target = await this.client.users.fetch(c.targetId);

            const message = await channel.messages.fetch(c.message);
            return await message.edit({
                embeds: [new MessageEmbed(message.embeds[0]).setDescription(`Member: \`${target.tag}\`
            Action: \`${c.action}\`
            Reason: \`${reason}\`
            ${c.duration ? `Duration: \`${c.duration}\`` : ""}`)]
            })
        } catch (err: any) {
            return this.client.logger.error`There was an error editing a cases reason with the id ${id}`
        }
    }
}
