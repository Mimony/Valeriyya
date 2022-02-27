import type {
    ApplicationCommandData,
    CommandInteraction,
    ContextMenuInteraction,
    InteractionReplyOptions,
    MessagePayload
} from "discord.js";

// read only array of any types
type Arr = readonly any[];

// A generic constructor with params
export type Ctor<A extends Arr = readonly any[], R = any> = new (...args: A) => R;

// A generic abstract constructor with params
export type ACtor<A extends Arr = readonly any[], R = any> = abstract new (...args: A) => R;

// A generic constructor without params
export type Constructor<T> = new (...args: any[]) => T;

// A generic abstract constructor without params
export type AbstractConstructor<T> = abstract new (...args: any[]) => T;

// Some ppl say this is the biggest mistake in the Javascript ecosystem so will i so there it is
export type Nullish = null | undefined;

export interface ICommandInteraction extends CommandInteraction { }
export interface IContextInteraction extends ContextMenuInteraction { }

export type ICommandExecute = (interaction: CommandInteraction) => Promise<string | MessagePayload | InteractionReplyOptions | void> | InteractionReplyOptions | string | void;
export type IContextExecute = (interaction: ContextMenuInteraction) => Promise<string | MessagePayload | InteractionReplyOptions | void> | InteractionReplyOptions | string | void;

export function isNullish(value: unknown): value is Nullish {
    return value === undefined || value === null;
}

export interface ICommand {
    chat?: ICommandExecute;
    menu?: IContextExecute;
    data: ApplicationCommandData;
}

export const OptionTypes = {
    SUB_COMMAND: 1,
    SUB_COMMAND_GROUP: 2,
    STRING: 3,
    INTEGER: 4,
    BOOLEAN: 5,
    USER: 6,
    CHANNEL: 7,
    ROLE: 8,
    MENTIONABLE: 9,
    NUMBER: 10,
};

export const AppOptionTypes = {
    CHAT_INPUT: 1,
    USER: 2,
    MESSAGE: 3
};

export const defineCommand = (cmd: ICommand): ICommand => cmd;
export interface Case {
    message?: string;
    id: number;
    action: "ban" | "kick" | "mute" | "unban" | "unmute";
    guildId: string;
    staffId: string;
    targetId: string;
    date: number;
    reason: string;
    duration: number | 0;
};

export interface Channels {
    logs: string;
    welcome: string;
};

export interface Roles {
    staff: string;
    mute: string;
};

export interface History {
    id: string;
    ban: number;
    kick: number;
    mute: number;
}

export type IGuildDb = {
    gid: string;
    cases: object[];
    cases_number: number;
    channels: object;
    roles: object;
    history: object[];
};

export type GuildDb = {
    gid: string;
    cases: Case[];
    cases_number: number;
    channels: Channels;
    roles: Roles;
    history: History[];
};
