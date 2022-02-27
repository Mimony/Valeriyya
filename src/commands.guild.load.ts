import { REST } from "@discordjs/rest";
import { Routes } from "discord-api-types/v9";
import { Collection } from "discord.js";
import type { ICommand } from "./lib/util/valeriyya.types";
import { Logger } from "./lib/util/valeriyya.logger";
import { Commands } from "./commands";

const clientID = "830130301535649853";
const guildID = "525322311826669569";
const logger: Logger = new Logger();

const commands: Collection<string, ICommand> = new Collection();

function loadCommands() {
    logger.print("Loading commands");

    for (const command of Commands()) {
        commands.set(command.data.name, command);
    }
}

const rest = new REST({version: "9"}).setToken("ODMwMTMwMzAxNTM1NjQ5ODUz.YHCNFg.RkSaienjc7hLaWpRx-XjzCST4pk");

(async () => {
    try {
        loadCommands();

        logger.print("Loading Application Commands....");

        await rest.put(Routes.applicationGuildCommands(clientID, guildID), {body: commands.map(c => c.data)});

        logger.print("Finished Loading Application Commands.");

    } catch (err: any) {
        logger.error(err);
    }
})()