import client from "./index";
import { REST } from "@discordjs/rest";
import { Routes } from "discord-api-types/v9";

const clientID = "909791454040301568";
const guildID = "909850768947937290"

const rest = new REST({ version: "9" }).setToken("OTA5NzkxNDU0MDQwMzAxNTY4.YZJbUQ.c8PIUM_EftouBg9KKV9bDG6IWCY");

(async () => {
    try {
        client.logger.print("Loading Application Commands....");

        await rest.put(Routes.applicationGuildCommands(clientID, guildID), { body: client.commands.map(c => c.data) });

        client.logger.print("Finished Loading Application Commands.");

    } catch(err: any) {
        client.logger.error(err);
    }
})()