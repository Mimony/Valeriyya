import { BaseEntity, Column, ObjectID, ObjectIdColumn, PrimaryColumn } from "typeorm";
import { Logger } from "./valeriyya.logger";

export class GuildEntity extends BaseEntity {
    @ObjectIdColumn({ name: "_id" })
    public _id!: ObjectID;

    @PrimaryColumn({ name: "id", type: "string", unique: true, update: false })
    public id!: string;

    @Column({ name: "cases", array: true, nullable: false, default: [] })
    public cases!: Case[];

    @Column({ name: "cases.number", type: "number", nullable: false, default: 0 })
    public cases_number!: number;

    @Column({ name: "roles", nullable: true })
    public roles?: {
        staff?: string;
        mute?: string;
    };

    @Column({ name: "channels", nullable: true })
    public channels?: {
        logs?: string;
        welcome?: string;
    }

    public logger: Logger = new Logger()

    public constructor(guild: string) {
        super();
        this.id = guild
    }

    public getCaseById(id: number) {
        const c = this.cases.find(c => c.id === id);
        if (c) return c;
        else return this.logger.error(`There is no such case with the id: ${id}`)
    }

    public getCasesByAction(action: "ban" | "kick" | "mute" | "unban" | "unmute") {
        const c = this.cases.filter(c => c.action === action);
        if (c.length > 0) return c;
        else return this.logger.error(`There is no cases with a ${action} action.`)
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
    reason: string | "No reason!";
    duration: number | 0;
}