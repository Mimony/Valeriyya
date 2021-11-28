import type { Valeriyya } from "#lib/ValeriyyaClient";
import type { ApplicationCommandOptionData, Awaitable, CommandInteraction } from "discord.js";

export abstract class Command<O extends Command.Options = Command.Options> {

    public name: string;
    public description: string;
    public client?: Valeriyya;

    public constructor(options: Command.Options = {}) {
        this.name = options.name ?? '';
        this.description = options.description ?? '';
    }

    public abstract execute(interaction: CommandInteraction): Awaitable<unknown>;
    
}

export interface CommandOption {
    name?: string;
    description?: string;
    data?: ApplicationCommandOptionData[];
    defaultPermission?: boolean;
}

export namespace Command {
    export type Options = CommandOption;
}