import { GuildMember, MessageEmbed, TextBasedChannel, User } from "discord.js";
import type { Valeriyya } from "../valeriyya.client";
import type { Case } from "./valeriyya.db.models";
import { ValeriyyaEmbed } from "./valeriyya.embed";
import ms from "./valeriyya.ms";

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
      const channel = (await guild.channels.fetch(channelId)) as Omit<TextBasedChannel, "DMChannel" | "PartialDMChannel" | "ThreadChannel">;

      message = (
        await channel.send({
          embeds: [
            await this.log({
              action,
              staff,
              target,
              id,
              reason,
              duration,
              date,
            }),
          ],
        })
      ).id;
    } catch (err: any) {
      this.client.logger.error(`There was an error logging the case: ${err}`);
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
      duration,
    };

    db.cases.push(new_case);
    return db.save();
  }

  public async log({
    action,
    staff,
    target,
    id,
    reason,
    duration,
    date,
  }: {
    action: "ban" | "kick" | "mute" | "unban" | "unmute";
    staff: GuildMember;
    target: User;
    id: number;
    reason: string;
    duration?: number;
    date: number;
  }): Promise<ValeriyyaEmbed> {
    return new ValeriyyaEmbed()
      .setAuthor({ name: `${staff.user.tag} (${staff.user.id})`, iconURL: staff.user.displayAvatarURL({ dynamic: true }) })
      .setFooter(`Case: ${id}`)
      .setDescription(
        `Member: \`${target.tag}\`
            Action: \`${action}\`
            Reason: \`${reason}\`
            ${duration ? `Duration: \`${ms(duration, true)}\`` : ""}
            `
      )
      .setTimestamp(date);
  }

  public async edit({ guildId, id, reason, action }: { guildId: string; id: number; reason?: string; action?: "ban" | "kick" | "mute" | "unban" | "unmute" }) {
    const db = await this.client.db(guildId);
    const guild = await this.client.guilds.fetch(guildId);

    const c = db.getCaseById(id);
    if (!c) return `There is no such case with the id ${id}`;
    if (reason) c.reason = reason;
    if (action) c.action = action;
    db.save();

    if (!c.message) return;
    const channel_id = db.channels.logs;
    try {
      const channel = (await guild.channels.fetch(channel_id!)) as Omit<TextBasedChannel, "DMChannel" | "PartialDMChannel" | "ThreadChannel">;
      const target = await this.client.users.fetch(c.targetId);

      const message = await channel.messages.fetch(c.message);
      return await message.edit({
        embeds: [
          new MessageEmbed(message.embeds[0]).setDescription(`Member: \`${target.tag}\`
            Action: \`${action}\`
            Reason: \`${reason}\`
            ${c.duration ? `Duration: \`${ms(c.duration, true)}\`` : ""}`),
        ],
      });
    } catch (err: any) {
      return this.client.logger.error`There was an error editing a cases reason with the id ${id}`;
    }
  }
}
