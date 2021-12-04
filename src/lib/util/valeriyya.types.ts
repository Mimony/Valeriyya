import type { ApplicationCommandData, CommandInteraction, InteractionReplyOptions, MessagePayload } from "discord.js";

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

export interface ICommandInteraction extends CommandInteraction {
    // db: GuildDb 
}
export type ICommandExecute = (interaction: CommandInteraction) => Promise<string | MessagePayload | InteractionReplyOptions | void> | InteractionReplyOptions | string| void;

export interface ICommand {
    execute: ICommandExecute;
    data: ApplicationCommandData
}

export const defineCommand = (cmd: ICommand): ICommand => cmd;