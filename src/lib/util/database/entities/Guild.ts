import { PrimaryColumn, ObjectID, Column, Entity, BaseEntity, ObjectIdColumn } from "typeorm";
import { Valeriyya } from "../../../valeriyya.client";
import type { Case, Channels, History, Roles } from "../../valeriyya.types";

@Entity("guild")
export class GuildDb extends BaseEntity {

    private logger = Valeriyya.initLogger();

    @ObjectIdColumn({ name: "_id"})
    _id!: ObjectID;

    @PrimaryColumn()
    id: string;

    @Column()
    cases: {
        total: number,
        case: Case[]
    } = {
        total: 0,
        case: []
    };

    @Column()
    roles: Roles = {
        staff: undefined,
        mute: undefined
    };

    @Column()
    channels: Channels = {
        logs: undefined,
        welcome: undefined
    };

    @Column()
    history: History[] = [];

    public constructor(id: string) {
        super();
        this.id = id;
    }

    public getCase(id: number) {
        const c = this.cases.case.find(c => c.id === id);

        if (!c) return this.logger.print`There is no such case with the case-id ${id} in guild ${this.id}`;

        return c;
    }

    public deleteCase(id: number) {
        const c = this.cases.case.find(c => c.id === id);
        
        if (!c) return this.logger.print`There is no such case with the case-id ${id} in guild ${this.id}`;

        const index = this.cases.case.indexOf(c);
        this.cases.case.slice(index, 1);
        this.save();
    }

    public getUserHistory(id: string) {
        let history = this.history.find(u => u.id === id);

        if (!history) {
            this.history.push({ id, ban: 0, kick: 0, mute: 0 });
            this.save().then(db => history = db.history.find(u => u.id === id));
        }

        return history!;
    }
}