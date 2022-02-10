import { Action, ActionData, getUserHistory, Moderation } from "./valeriyya.moderation";
import { ValeriyyaEmbed } from "../valeriyya.embed";
import { reply } from "../valeriyya.util";

type KickData = Omit<ActionData, "duration">;

export class Kick extends Moderation {
  public constructor(data: KickData) {
    super(Action.KICK, data);
  }

  public permissions() {
    if (!this.int.memberPermissions?.has("KICK_MEMBERS", true)) {
      const embed = new ValeriyyaEmbed(undefined, "error")
        .setAuthor({ name: `${this.int.user.tag} (${this.int.user.id})`, url: this.int.user.displayAvatarURL({ dynamic: true }) })
        .setDescription("You are missing the `KICK_MEMBERS` permission");
      reply(this.int, { embeds: [embed] });
      return false;
    }
    return true;
  }

  public async execute(): Promise<boolean> {
    const db = await this.client.guild.get(this.int.guildId!);
    const history_number = getUserHistory({ client: this.client, db, id: this.target.id })!.kick + 1;
    const cases_number = db.cases_number + 1;

    try {
      await this.int.guild?.members.kick(this.target, `Case ${cases_number}`);
    } catch (e: any) {
      reply(this.int, { content: `There was an error kicking this member: ${e}`, ephemeral: true });
      this.client.logger.error(`There was an error with the moderation-KICK method: ${e}`);
      
      return false;
    }

    db.cases_number = cases_number;
    db.history.find((m) => m.id === this.target.id)!.kick = history_number;
    this.client.guild.set(this.int.guildId!, db)

    return true;
  }
}
