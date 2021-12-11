import { REST } from "@discordjs/rest";
import { Routes } from "discord-api-types/v9";
import { Collection } from "discord.js";
import type { ICommand } from "./lib/util/valeriyya.types";
import { Logger } from "./lib/util/valeriyya.logger";
import { Commands } from "./commands";

const clientID = "909791454040301568";
const guildID = "909850768947937290";
const logger: Logger = new Logger();

const commands: Collection<string, ICommand> = new Collection();

function loadCommands() {
    logger.print("Loading commands");

    for (const command of Commands()) {
        commands.set(command.data.name, command);
    }
}

const rest = new REST({version: "9"}).setToken("OTA5NzkxNDU0MDQwMzAxNTY4.YZJbUQ.c8PIUM_EftouBg9KKV9bDG6IWCY");

(async () => {
    try {
        await loadCommands();

        logger.print("Loading Application Commands....");

        await rest.put(Routes.applicationGuildCommands(clientID, guildID), {body: commands.map(c => c.data)});

        logger.print("Finished Loading Application Commands.");

    } catch (err: any) {
        logger.error(err);
    }
})()