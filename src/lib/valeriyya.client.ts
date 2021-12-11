import { Commands } from "../commands";
import { Client, Collection, Interaction } from "discord.js";
import { Logger } from "./util/valeriyya.logger";
import type { ICommand } from "./util/valeriyya.types";
import { ValeriyyaDB } from "./util/valeriyya.db";

const uri = "mongodb+srv://Client:MomsSpaghetti@cluster0.i1oux.mongodb.net/myFirstDatabase?retryWrites=true&w=majority"

declare module "discord.js" {
    interface Client {
        logger: Logger;
        commands: Collection<string, ICommand>;
    }
}

export class Valeriyya extends Client {
    public commands: Collection<string, ICommand> = new Collection();
    public logger: Logger = new Logger();
    public db: ValeriyyaDB = new ValeriyyaDB(this);

    public constructor() {
        super({
            intents: [
                "GUILDS",
                "GUILD_MEMBERS",
                "GUILD_MESSAGES",
                "GUILD_MESSAGE_REACTIONS"
            ]
        })

        this.on("ready", () => this.onReady());
        this.on("interactionCreate", (interaction) => this.onInteraction(interaction) )

    }

    public async start(token: string): Promise<string> {
        return super.login(token)
    }

    private async onReady() {
        await this.db.init(uri);

        await this.loadCommands();
        this.logger.print(`${this.user?.tag} is ready to shine.`)
    }

    private async onInteraction(interaction: Interaction) {
        if (!interaction.isCommand() || !interaction.inGuild() || !interaction.guild?.available) return;

        const command = this.commands.get(interaction.commandName);
        if (!command) return;

        try {
            var result = await command.execute(interaction)
        } catch(err: any) {
            interaction.replied ? 
            interaction.followUp({ content: `There was an error ${err.message}`, ephemeral: true }) :
            interaction.reply({ content: `There was an error ${err.message}`, ephemeral: true });
        }
        
        if (!result) return;
        
        interaction.replied ?
        interaction.followUp(result) :
        interaction.reply(result);

    } 

    private async loadCommands() {
        this.logger.print("Loading commands");

        for (const command of Commands()) {
            this.commands.set(command.data.name, command)
        }
    }
}