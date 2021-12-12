import type { Valeriyya } from "../valeriyya.client";
import * as mongodb from "mongodb";
import type { Nullish } from "./valeriyya.types";
import { Guild, type IGuild, type IGuildSearch } from "./valeriyya.db.models";

export class ValeriyyaDB {

    public client: Valeriyya;
    public db: mongodb.Db | Nullish;

    public constructor(client: Valeriyya) {
        this.client = client;
    }

    public async init(uri: string): Promise<mongodb.Db> {
        const db_client = new mongodb.MongoClient(uri);
        await db_client.connect();
        this.client.logger.print("Database has been initialized!")

        this.db = db_client.db("Main");
        return this.db;
    }

    public async get(guild: string) {
        const db = this.db?.collection<IGuild>("Guild");
        let guild_db = await db?.findOne<IGuildSearch>({id: guild})

        if (!guild_db) {
            await db?.insertOne(new Guild(guild))
        }

        return guild_db;
    }
}