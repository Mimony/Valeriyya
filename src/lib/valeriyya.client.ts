import { Commands } from "../commands";
import { Client, Collection, Guild, Interaction, Message } from "discord.js";
import { Logger } from "./util/valeriyya.logger";
import { ValeriyyaDB } from "./util/database/valeriyya.db";
import { ValeriyyaCases } from "./util/valeriyya.cases";
import { reply, replyC } from "./util/valeriyya.util";
import play from "play-dl";
import type { ICommand, ICommandInteraction, IContextInteraction } from "./util/valeriyya.types";
import type { MusicSubscription } from "./util/valeriyya.music";
import { GuildDb } from "./util/database/entities/Guild";
import { DataSource } from "typeorm";

let count: number = 0;
declare module "discord.js" {
  interface Client {
    logger: Logger;
    commands: Collection<string, ICommand>;
    db: ValeriyyaDB;
    cases: ValeriyyaCases;
    settings(int: ICommandInteraction | IContextInteraction | Guild): Promise<GuildDb>;
    subscription: Collection<string, MusicSubscription>;
  }

  interface Message {
    client: Valeriyya;
  }

  interface CommandInteraction {
    client: Valeriyya;
  }

  interface ContextMenuInteraction {
    client: Valeriyya;
  }
}

export class Valeriyya extends Client {
  public commands: Collection<string, ICommand> = new Collection();
  public db: ValeriyyaDB = new ValeriyyaDB(this);
  public cases: ValeriyyaCases = new ValeriyyaCases(this);

  public static initLogger(): Logger {
    return new Logger();
  }

  public logger: Logger = new Logger();

  public constructor() {
    super({
      intents: ["GUILDS", "GUILD_MEMBERS", "GUILD_VOICE_STATES"],
    });

    this.on("ready", () => this.onReady());
    this.on("interactionCreate", (interaction) => this.onInteraction(interaction));
    this.on("messageCreate", async (message) => this.onMessageAnnoyFriend(message));
  }

  public async start(token: string): Promise<string> {
    // await this.db.on();
    const connection = new DataSource({
      url: "mongodb+srv://Client:MomsSpaghetti@cluster0.i1oux.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
      type: "mongodb",
      // useUnifiedTopology: true,
      entities: [GuildDb],
    })

    await connection.initialize();

    console.log(connection.getRepository("guild"))
    if(connection.isInitialized) this.logger.print`The connection to the database has been established.`
    else this.logger.print`The database connection has failed!`;
    
    this.logger.print("Booting up....");
    return super.login(token);
  }

  public async settings(argument: ICommandInteraction | IContextInteraction | Guild): Promise<GuildDb> {
    let id: string;
    if (argument instanceof Guild) id = argument.id;
    else id = argument.guildId!;

    let db = await GuildDb.findOne({ where: { id: id } })

    if (!db) {
      db = new GuildDb(id);
      db.save();
    }

    return db;
  }

  private async onReady() {
    await play.setToken({
      spotify: {
        client_id: "15fdd20340ff417ba4b7bf2c8bdca07b",
        client_secret: "04421c834d5d42efb122db7b69cbc108",
        refresh_token: "AQBN-7v23aiWf339Pe0BbRY966oba-V_GuucfaYNUapr5a-d1u0qfNC1vXW7GLPrt0Va9eU0He14R1LVq2LOCHeV95e7Y3gdjvii-MeM1OkUXv3LynxGS4IznbWWw2c3f70",
        market: "MK",
      },
    });

    await this.loadCommands();
    this.logger.print(`${this.user?.tag} is ready to shine.`);
  }

  private async onInteraction(interaction: Interaction) {
    if (!interaction.inGuild() || !interaction.guild?.available) return;

    let result;
    if (interaction.isCommand()) {
      const command = this.commands.get(interaction.commandName);
      if (!command) return;

      try {
        result = await command.chat!(interaction);
        this.logger.print(`${interaction.user.tag} ran ${interaction.commandName}`);
      } catch (err: any) {
        reply(interaction, { content: `There was an error ${err.message}`, ephemeral: true });
      }

      if (!result) return;

      reply(interaction, result);
    } else if (interaction.isContextMenu()) {
      const command = this.commands.get(interaction.commandName);
      if (!command) return;

      try {
        result = await command.menu!(interaction);
        this.logger.print(`${interaction.user.tag} ran ${interaction.commandName}`);
      } catch (err: any) {
        replyC(interaction, { content: `There was an error ${err.message}`, ephemeral: true });
      }

      if (!result) return;

      replyC(interaction, result);
    }
  }

  private async onMessageAnnoyFriend(message: Message) {
    if (message.author.bot || message.channel.type === "DM") return;
    if (["736370979026108517", "509732641369620501"].some((id) => message.author.id !== id)) return;

    if (["sus", "amogus", "amogus", "amo", "gus"].some((word) => message.content.includes(word))) {
      setTimeout(() => message.delete(), 1500);
      count += 1;
      console.log(`Deleted ${message.author.tag} message. Messages deleted so far: ${count}`);
    }
  }

  private async loadCommands() {
    this.logger.print("Loading commands");

    for (const command of Commands()) {
      this.commands.set(command.data.name, command);
    }
  }
}
