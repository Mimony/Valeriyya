import { Commands } from "../commands";
import { Client, Collection, Guild, Interaction, Message } from "discord.js";
import { Logger } from "./util/valeriyya.logger";
import type { ICommand } from "./util/valeriyya.types";
import { ValeriyyaDB } from "./util/valeriyya.db";
import { GuildEntity } from "./util/valeriyya.db.models";
import { ValeriyyaCases } from "./util/valeriyya.cases";
import { reply, replyC } from "./util/valeriyya.util";

const uri: string = "mongodb+srv://Client:MomsSpaghetti@cluster0.i1oux.mongodb.net/myFirstDatabase?retryWrites=true&w=majority";
let count: number = 0;
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
      intents: ["GUILDS", "GUILD_MEMBERS"],
    });

    this.on("ready", () => this.onReady());
    this.on("interactionCreate", (interaction) => this.onInteraction(interaction));
    this.on("messageCreate", async (message) => this.onMessageAnnoyFriend(message))
  }

  public async start(token: string): Promise<string> {
    this.logger.print("Booting up....");
    return super.login(token);
  }

  public async db(guild: Guild | string): Promise<GuildEntity> {
    let g: string;
    guild instanceof Guild ? (g = guild.id) : (g = guild);

    let db = await GuildEntity.findOne({ id: g });
    if (!db) {
      db = new GuildEntity(g);
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
        var result = await command.chat!(interaction);
        this.logger.print(`${interaction.user.tag} ran ${interaction.commandName}`);
      } catch (err: any) {
          reply(interaction, { content: `There was an error ${err.message}`, ephemeral: true })
      }

      if (!result) return;

      reply(interaction, result)
    } else if (interaction.isContextMenu()) {
      const command = this.commands.get(interaction.commandName);
      if (!command) return;

      try {
        var result = await command.menu!(interaction);
        this.logger.print(`${interaction.user.tag} ran ${interaction.commandName}`);
      } catch (err: any) {
        replyC(interaction, { content: `There was an error ${err.message}`, ephemeral: true })
      }

      if (!result) return;

      replyC(interaction, result)
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
