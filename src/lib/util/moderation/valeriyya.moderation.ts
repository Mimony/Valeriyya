import type { CommandInteraction, GuildMember, User } from "discord.js";
import type { Valeriyya } from "../../valeriyya.client";

export enum Action {
    BAN = "ban",
    UNBAN = "unban",
    KICK = "kick",
    MUTE = "mute",
    UNMUTE = "unmute"
}

export interface ActionData {
    int: CommandInteraction;
    staff: GuildMember;
    target: GuildMember | User;
    date: Date;
    reason?: string;
    duration?: number;
}
export abstract class Moderation {
    protected client: Valeriyya;
    protected int: CommandInteraction; 
    protected staff: GuildMember;
    protected target: GuildMember | User;
    protected reason?: string;
    protected date: Date;
    protected duration: number;
    public constructor(protected action: Action, data: ActionData) {
        this.int = data.int;
        this.client = this.int.client as Valeriyya;
        this.staff = data.staff;
        this.target = data.target;
        this.reason = data.reason ?? "Unreasonable";
        this.date = data.date;
        this.duration = data.duration ?? 0;
    }

    public abstract permissions(): Promise<void>;
    public abstract execute(): Promise<void>;
    public abstract db(): Promise<void>;

    public async all() {
        try {
            await this.permissions();
            await this.execute();
            await this.db();
        } catch (err: any) {
            this.client.logger.error(`There was an error executing the moderation action: ${err}`);
        }
    }

}