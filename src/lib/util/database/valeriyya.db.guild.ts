import type { PrismaClient } from "@prisma/client";
import type { IGuildDb, GuildDb } from "../valeriyya.types";

export class ValeriyyaGuildDb {
    public constructor(private readonly db: PrismaClient['guild']) { }

    public async get(guildId: string): Promise<GuildDb> {
        const guild = await this.db.findUnique({ where: { gid: guildId } });
        if (!guild) return this._generate({
            gid: guildId,
            cases_number: 0,
            cases: [],
            history: [],
            roles: {
                staff: null,
                mute: null,
            },
            channels: {
                welcome: null,
                logs: null,
            }
        });
        else return guild as unknown as GuildDb;
    }

    public set(guildId: string, data: IGuildDb) {
        this.db.update({
            where: {
                gid: guildId
            },
            data
        });
    }

    private _generate(data: IGuildDb): GuildDb {
        const db = this.db.create({ data }) as unknown as GuildDb;
        return db;
    }
}