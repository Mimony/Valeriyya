import { Action, ActionData, Moderation } from "./valeriyya.moderation"
import { ValeriyyaEmbed } from "../valeriyya.embed";
import { User } from "discord.js";

type MuteData = ActionData;

export class Mute extends Moderation {
    public constructor(data: MuteData) {
        super(Action.MUTE, data)
    }

    public permissions() {
        if (!this.int.memberPermissions?.has("MODERATE_MEMBERS", true)) {
            const embed = new ValeriyyaEmbed(undefined, "error")
                .setAuthor({name: `${this.int.user.tag} (${this.int.user.id})`, url: this.int.user.displayAvatarURL({ dynamic: true }) })
                .setDescription("You are missing the `TIMEOUT_MEMBERS` permission");
            if (!this.int.replied) this.int.reply({embeds: [embed]})
            else this.int.followUp({embeds: [embed]});
            return false;
        }
        return true;
    }

    public async execute(): Promise<void> {
        if (this.target instanceof User) return;
        const db = await this.client.db(this.int.guild!)
        const history_number = (await db.getUserHistory(this.target.id))!.mute + 1;
        const cases_number = db.cases_number + 1;

        try {
            await this.target.timeout(this.duration, `Case ${cases_number}`)
        } catch (e: any) {
            if (!this.int.replied) this.int.reply({
                content: `There was an error muting this member: ${e}`,
                ephemeral: true
            });
            else this.int.followUp({ content: `There was an error muting this member: ${e}`, ephemeral: true });
            this.client.logger.error(`There was an error with the moderation-MUTE method: ${e}`);
        }

        db.cases_number = cases_number;
        db.history.find(m => m.id === this.target.id)!.mute = history_number;
        await db.save();
    }
}