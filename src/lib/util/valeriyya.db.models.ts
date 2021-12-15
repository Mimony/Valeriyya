import { BaseEntity, Column, Entity, ObjectID, ObjectIdColumn, PrimaryColumn } from "typeorm";
import { Logger } from "./valeriyya.logger";

const logger: Logger = new Logger()
@Entity("GuildEntity")
export class GuildEntity extends BaseEntity {
    @ObjectIdColumn({ name: "_id" })
    public _id!: ObjectID;

    @PrimaryColumn({ name: "id", type: "string", unique: true, update: false })
    public id!: string;

    @Column({ name: "cases", array: true, nullable: false, default: [] })
    public cases!: Case[];

    @Column({ name: "cases.number", type: "number", nullable: false, default: 0 })
    public cases_number!: number;

    @Column({ name: "roles" })
    public roles: {
        staff: string | null;
        mute: string | null;
    } = {
        staff: null,
        mute: null
    }

    @Column({ name: "channels" })
    public channels: {
        logs: string | null;
        welcome: string | null;
    } = {
        logs: null,
        welcome: null
    }

    public constructor(guild: string) {
        super();
        this.id = guild
    }

    public getCaseById(id: number) {
        const c = this.cases.find(c => c.id === id);
        if (c) return c;
        else return logger.error(`There is no such case with the id: ${id}`)
    }

    public getCasesByAction(action: "ban" | "kick" | "mute" | "unban" | "unmute") {
        const c = this.cases.filter(c => c.action === action);
        if (c.length > 0) return c;
        else return logger.error(`There is no cases with a ${action} action.`)
    }

    public addCase({ message, id, action, guildId, staffId, targetId, date, reason, duration }: Case) {
        this.cases.push({
            message,
            id,
            action,
            guildId,
            staffId,
            targetId,
            date,
            reason,
            duration
        });
        this.cases_number ++;

        return this.save();
    }

    public removeCase(id: number) {
        const index = this.getCaseById(id)!;
        this.cases.splice(this.cases.indexOf(index), 1);
        this.cases_number --;

        return this.save();
    }
}

export interface Case {
    message?: string;
    id: number;
    action: "ban" | "kick" | "mute" | "unban" | "unmute";
    guildId: string;
    staffId: string;
    targetId: string;
    date: Date;
    reason: string;
    duration: number | 0;
}