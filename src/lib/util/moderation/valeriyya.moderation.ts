import type { GuildMember, User } from "discord.js";
import type { Valeriyya } from "../../valeriyya.client";
import type { ValeriyyaSettings } from "../database/valeriyya.db.settings";
import { Case, History, ICommandInteraction, isNullish } from "../valeriyya.types";

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

export async function getUserHistory ({ gid, id, db }: { gid: string; id: string; db: ValeriyyaSettings; }) {
    const db_history = await db.get(gid, "history") as History[];

    let history;
    let history_find = db_history.find((m) => m.id === id);

    if (isNullish(history_find)) {
        await db.set(gid, "history", [{
            id,
            ban: 0,
            kick: 0,
            mute: 0
        }]);
        history = db_history.find((m) => m.id === id);
    }
    console.log(history);
    return history;
}

export async function getCaseById ({ gid, id, db, client }: { gid: string; id: number; db: ValeriyyaSettings; client: Valeriyya; }) {
    const cases = await db.get(gid, "cases") as Case[];
    const c = cases.find((c) => c.id === id);
    if (!c) return client.logger.print`There is no such case with the id: ${id}`;
    return c;
}

export async function deleteCaseById ({ gid, id, db, client }: { gid: string; id: number; db: ValeriyyaSettings; client: Valeriyya; }) {
    const cases = await db.get(gid, "cases") as Case[];
    const c = cases.find((c) => c.id === id);
    if (!c) return client.logger.print`There is no such case with the id ${id}`;

    client.settings.delete(gid, "cases", c);
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

    protected async reason () {
        if (this._reason) return this._reason;
        const cases = await this.client.settings.get(this.int.guildId!, "cases.total");
        return `\`Use /reason ${cases} <...reason> to set a reason for this case.\``;
    }

    public abstract permissions (): boolean;
    public abstract execute (): Promise<boolean>;

    public async db (): Promise<any> {
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

    public async all () {
        try {
            if (!this.permissions()) return;
            if (!this.execute()) return;
            else this.execute();
            await this.db();
        } catch (err: any) {
            this.client.logger.error(`There was an error executing the moderation action: ${err.stack}`);
        }
    }

}