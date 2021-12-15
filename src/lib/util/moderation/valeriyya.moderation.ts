import type { GuildMember, User } from "discord.js";
import type { Valeriyya } from "../../valeriyya.client";
import type { ICommandInteraction } from "../valeriyya.types";

export enum Action {
    BAN = "ban",
    UNBAN = "unban",
    KICK = "kick",
    MUTE = "mute",
    UNMUTE = "unmute"
}

export interface ActionData {
    int: ICommandInteraction;
    staff: GuildMember;
    target: GuildMember | User;
    date: number;
    reason: string;
    duration?: number;
}

export abstract class Moderation {
    protected client: Valeriyya;
    protected int: ICommandInteraction;
    protected staff: GuildMember;
    protected target: GuildMember | User;
    protected _reason: string;
    protected date: number;
    protected duration: number;

    public constructor(protected action: Action, data: ActionData) {
        this.int = data.int;
        this.client = this.int.client as Valeriyya;
        this.staff = data.staff;
        this.target = data.target;
        this._reason = data.reason;
        this.date = data.date;
        this.duration = data.duration ?? 0;
    }

    protected async reason() {
        if (this._reason) return this._reason;
        const db = await this.client.db(this.int.guild!);
        const cases = db.cases_number;
        return `\`Use /reason ${cases} <...reason> to set a reason for this case.\``;
    }

    public abstract permissions(): boolean;
    public abstract execute(): Promise<any>;

    public async db(): Promise<void> {
        return this.client.cases.add({
            guildId: this.int.guild!.id,
            staffId: this.staff.id,
            targetId: this.target.id,
            action: this.action,
            date: this.date,
            reason: await this.reason(),
            duration: this.duration
        });
    }

    public async all() {
        try {
            if (!this.permissions()) return;
            await this.execute();
            await this.db();
        } catch (err: any) {
            this.client.logger.error(`There was an error executing the moderation action: ${err.stack}`);
        }
    }

}