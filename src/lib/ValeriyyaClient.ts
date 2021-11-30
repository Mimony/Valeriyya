import { Client, Collection, type CommandInteraction } from "discord.js";
import { Fragments } from "./structures/Fragment";

export class Valeriyya extends Client {
    public commands: Collection<string, (interaction: CommandInteraction) => void>= new Collection();
    public fragments: Fragments = new Fragments(this);

    public constructor() {
        super({
            intents: [
                "GUILDS",
                "GUILD_MEMBERS",
                "GUILD_MESSAGES",
                "GUILD_MESSAGE_REACTIONS"
            ]
        })
    }

    public async start(token: string): Promise<string> {
        this.fragments.loadListeners();
        return super.login(token)
    }
}