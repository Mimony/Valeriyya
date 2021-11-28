import { Client } from "discord.js";

export class Valeriyya extends Client {
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
        return super.login(token)
    }
}