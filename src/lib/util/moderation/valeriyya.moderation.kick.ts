import {Action, ActionData, Moderation} from "./valeriyya.moderation"
import {ValeriyyaEmbed} from "../valeriyya.embed";

type KickData = Omit<ActionData, "duration">;

export class Kick extends Moderation {
    public constructor(data: KickData) {
        super(Action.KICK, data)
    }

    public permissions() {
        if (!this.int.memberPermissions?.has("KICK_MEMBERS", true)) {
            const embed = new ValeriyyaEmbed("error")
                .setAuthor(`${this.int.user.tag} (${this.int.user.id})`, this.int.user.displayAvatarURL({dynamic: true}))
                .setDescription("You are missing the `KICK_MEMBERS` permission");
            if (!this.int.replied) this.int.reply({embeds: [embed]})
            else this.int.followUp({embeds: [embed]});
            return false;
        }
        return true;
    }

    public async execute(): Promise<void> {
        try {
            await this.int.guild?.members.kick(this.target)
        } catch (e: any) {
            if (!this.int.replied) this.int.reply({
                content: `There was an error kicking this member: ${e}`,
                ephemeral: true
            });
            else this.int.followUp({content: `There was an error kicking this member: ${e}`, ephemeral: true});
            this.client.logger.error(`There was an error with the moderation-KICK method: ${e}`);
        }
    }

    public db(): Promise<void> {
        throw new Error("Method not implemented.");
    }
}