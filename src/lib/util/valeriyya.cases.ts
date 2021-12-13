import type { Valeriyya } from "../valeriyya.client";
import type { ValeriyyaDB } from "./valeriyya.db";
import type { Cases } from "./valeriyya.db.models";

type Case = Omit<Cases, "id">;

export class ValeriyyaCases {
    public client: Valeriyya;
    public db: ValeriyyaDB;

    public constructor(client: Valeriyya) {
        this.client = client;
        this.db = client.db;
    }


    public async add({int, guildId, staffId, targetId, type, date, reason, duration}: Case) {
        const guild = await this.client.guilds.fetch(guildId);
        const staff = await guild.members.fetch(staffId);
        const target = await this.client.users.fetch(targetId);

        const db = await this.db.get(guildId);
        // const cases_number = db.case_number // get the cases_number number here

        // const cases = db.cases // get the cases to push to this array later on

        let new_case: Cases = {
            int,
            guildId,
            staffId,
            targetId,
            type,
            date,
            reason,
            duration
        };

        // cases.push(new_case); // push to the cases array of Cases
        // save changes to db
    }
}