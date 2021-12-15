import { BaseEntity, Column, ObjectID, ObjectIdColumn, PrimaryColumn } from "typeorm";
import { Logger } from "./valeriyya.logger";

export class GuildEntity extends BaseEntity {
    @ObjectIdColumn({name: "_id"})
    public _id!: ObjectID;

    @PrimaryColumn({name: "id", type: "string", unique: true, update: false})
    public id!: string;

    @Column({name: "cases", array: true, nullable: false, default: []})
    public cases!: Cases[];

    @Column({name: "cases.number", type: "number", nullable: false, default: 0})
    public cases_number!: number;

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

    public getCasesByAction(type: "ban" | "kick" | "mute" | "unban" | "unmute") {
        const c = this.cases.filter(c => c.type === type);
        if (c.length > 0) return c;
        else return this.logger.error(`There is no cases with a ${type} action.`)
    }

    public addCase({int, id, type, guildId, staffId, targetId, date, reason, duration}: Cases) {
        this.cases.push({
            int,
            id,
            type,
            guildId,
            staffId,
            targetId,
            date,
            reason,
            duration
        })

        return this.save();
    }

    public removeCase(id: number) {
        const index = this.getCaseById(id)!;
        this.cases.splice(this.cases.indexOf(index), 1);

        return this.save();
    }
}

export interface Cases {
    int?: string;
    id: number;
    type: "ban" | "kick" | "mute" | "unban" | "unmute";
    guildId: string;
    staffId: string;
    targetId: string;
    date: Date;
    reason: string | "No reason!";
    duration: number | 0;
}