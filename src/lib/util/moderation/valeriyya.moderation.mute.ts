import { Action, ActionData, Moderation } from "./valeriyya.moderation";
import { ValeriyyaEmbed } from "../valeriyya.embed";
import { User } from "discord.js";
import { reply } from "../valeriyya.util";

type MuteData = ActionData;

export class Mute extends Moderation {
  public constructor(data: MuteData) {
    super(Action.MUTE, data);
  }

  public permissions () {
    if (!this.int.memberPermissions?.has("MODERATE_MEMBERS", true)) {
      const embed = new ValeriyyaEmbed(undefined, "error")
        .setAuthor({ name: `${this.int.user.tag} (${this.int.user.id})`, url: this.int.user.displayAvatarURL({ dynamic: true }) })
        .setDescription("You are missing the `TIMEOUT_MEMBERS` permission");
      reply(this.int, { embeds: [embed] });
      return false;
    }
    return true;
  }

  public async execute (): Promise<boolean> {
    if (this.target instanceof User) return false;

    const db = await this.client.settings(this.int);
    const history = db.getUserHistory(this.target.id);
    const cases_number = db.cases.total + 1;

    try {
      await this.target.timeout(this.duration, `Case ${cases_number}`);
    } catch (e: any) {
      reply(this.int, { content: `There was an error muting this member: ${e}`, ephemeral: true });
      this.client.logger.error(`There was an error with the moderation-MUTE method: ${e}`);

      return false;
    }

    history.mute += 1;
    db.save();

    return true;
  }
}
