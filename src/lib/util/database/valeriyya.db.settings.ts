import type { Prisma, PrismaClient } from "@prisma/client";
import { Guild } from "discord.js";
import type { Valeriyya } from "../../valeriyya.client";

const defaultSettingsData = {
    "cases": [],
    "cases.total": 0,
    "role.mute": null,
    "role.mod": null,
    "channel.welcome": null,
    "channel.mod": null,
    "history": [],
}

export class ValeriyyaSettings {
    private client: Valeriyya;
    public db: PrismaClient;

    public constructor(client: Valeriyya) {
        this.client = client;
        this.db = this.client.prisma.dbClient;
    }

    public async get (guild: string | Guild, settingToGet: "role.mod" | "role.mute" | "channel.welcome" | "channel.mod" | "cases" | "cases.total" | "history") {
        let guildid: string;
        guild instanceof Guild ? guildid = guild.id : guildid = guild;

        let guildDB = await this.db.guild.findUnique({ where: { guildID: guildid } }) ||
        await this.db.guild.create({
            data: {
                guildID: guildid,
                settings: defaultSettingsData,
            }
        });
        

        let settingsObj = guildDB.settings as Prisma.JsonObject;
        if (settingsObj[settingToGet] instanceof Array) {
            if (settingToGet === "cases" || settingToGet === "history") return (settingsObj[settingToGet] as Array<Object>);
            return settingsObj[settingToGet] as any;
        }

        let settingFound = settingsObj[settingToGet];
        if (settingFound === null || settingFound === undefined) return settingFound = null;
        return settingFound;
    }


    public async set (guild: string | Guild, settingToChange: "role.mod" | "role.mute" | "channel.welcome" | "channel.mod" | "cases" | "cases.total" | "history", newValue: any, overide: boolean = false) {
        let guildid: string;
        guild instanceof Guild ? guildid = guild.id : guildid = guild;


        let guildDB = await this.db.guild.findUnique({ where: { guildID: guildid } }) ||
        await this.db.guild.create({
            data: {
                guildID: guildid,
                settings: defaultSettingsData,
            }
        });

        let settingsObj = guildDB.settings as Prisma.JsonObject;

        if (settingsObj[settingToChange] instanceof Array) {
            let setObj = settingsObj[settingToChange] as any[];
            if (newValue instanceof Array) {
                if (overide) {
                    settingsObj[settingToChange] = newValue;
                } else {
                    newValue.forEach(item => {
                        setObj.push(item);
                    });

                    settingsObj[settingToChange] = setObj;
                }
            } else {
                (settingsObj[settingToChange] as string[]).push(newValue);
            }
        } else settingsObj[settingToChange] = newValue;

        return await this.db.guild.update({
            where: {
                guildID: guildid
            },
            data: {
                settings: settingsObj
            }
        });
    }

    public async delete (guild: string | Guild, settingToRemove: "role.mod" | "role.mute" | "channel.welcome" | "channel.mod" | "cases" | "cases.total" | "history", valueToRemoveFromArray: any = "") {
        let guildid: string;
        guild instanceof Guild ? guildid = guild.id : guildid = guild;


        let guildDB = await this.db.guild.findUnique({ where: { guildID: guildid } }) ||
        await this.db.guild.create({
            data: {
                guildID: guildid,
                settings: defaultSettingsData,
            }
        });

        let settingsObj = guildDB.settings as Prisma.JsonObject;
        if (settingsObj[settingToRemove] instanceof Array) {
            let toRemoveIndex: number = (settingsObj[settingToRemove] as any[]).indexOf(valueToRemoveFromArray);
            if (toRemoveIndex === undefined) return this.client.logger.error('Couldn\'t find that value in the array you supplied!');
            (settingsObj[settingToRemove] as any[]).splice(toRemoveIndex, 1);
        } else delete settingsObj[settingToRemove];

        return await this.db.guild.update({
            where: {
                guildID: guildid
            },
            data: {
                settings: settingsObj
            }
        });
    }

}