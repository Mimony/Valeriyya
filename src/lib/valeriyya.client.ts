import { Commands } from "../commands";
import { Client, Collection, Guild, Interaction } from "discord.js";
import { Logger } from "./util/valeriyya.logger";
import type { ICommand } from "./util/valeriyya.types";
import { ValeriyyaDB } from "./util/valeriyya.db";
import { GuildEntity } from "./util/valeriyya.db.models";
import { ValeriyyaCases } from "./util/valeriyya.cases";

const uri = "mongodb+srv://Client:MomsSpaghetti@cluster0.i1oux.mongodb.net/myFirstDatabase?retryWrites=true&w=majority"

declare module "discord.js" {
    interface Client {
        logger: Logger;
        commands: Collection<string, ICommand>;
        db_init: ValeriyyaDB;

        db(guild: Guild | string): Promise<GuildEntity>;

        cases: ValeriyyaCases;
    }
}

export class Valeriyya extends Client {
    public commands: Collection<string, ICommand> = new Collection();
    public logger: Logger = new Logger();
    public db_init: ValeriyyaDB = new ValeriyyaDB(this);
    public cases: ValeriyyaCases = new ValeriyyaCases(this);

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
        this.on("interactionCreate", (interaction) => this.onInteraction(interaction))

    }

    public async start(token: string): Promise<string> {
        this.logger.print("Booting up....")
        return super.login(token)
    }

    public async db(guild: Guild | string): Promise<GuildEntity> {
        let g: string;
        guild instanceof Guild ?
            g = guild.id :
            g = guild;

        let db = await GuildEntity.findOne({id: g});
        if (!db) {
            db = new GuildEntity(g)
            return db.save();
        }
        return db;
    }

    private async onReady() {
        await this.db_init.on(uri);

        await this.loadCommands();
        this.logger.print(`${this.user?.tag} is ready to shine.`);
    }

    private async onInteraction(interaction: Interaction) {
        if (!interaction.inGuild() || !interaction.guild?.available) return;

        if (interaction.isCommand()) {
            const command = this.commands.get(interaction.commandName);
            if (!command) return;

            try {
                var result = await command.chat!(interaction)
                this.logger.print(`${interaction.user.tag} ran ${interaction.commandName}`)
            } catch (err: any) {
                interaction.replied || interaction.deferred ?
                    interaction.followUp({ content: `There was an error ${err.message}`, ephemeral: true }) :
                    interaction.reply({ content: `There was an error ${err.message}`, ephemeral: true });
            }

            if (!result) return;

            interaction.replied || interaction.deferred ?
                interaction.followUp(result) :
                interaction.reply(result);
        } else if (interaction.isContextMenu()) {
            const command = this.commands.get(interaction.commandName);
            if (!command) return;

            try {
                var result = await command.context!(interaction)
                this.logger.print(`${interaction.user.tag} ran ${interaction.commandName}`)
            } catch (err: any) {
                interaction.replied || interaction.deferred ?
                    interaction.followUp({ content: `There was an error ${err.message}`, ephemeral: true }) :
                    interaction.reply({ content: `There was an error ${err.message}`, ephemeral: true });
            }

            if (!result) return;

            interaction.replied || interaction.deferred ?
                interaction.followUp(result) :
                interaction.reply(result);
        }

    }

    private async loadCommands() {
        this.logger.print("Loading commands");

        for (const command of Commands()) {
            this.commands.set(command.data.name, command)
        }
    }
}