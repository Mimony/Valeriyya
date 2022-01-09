import type { CommandInteraction, ContextMenuInteraction, InteractionReplyOptions, MessagePayload } from "discord.js";

export function reply(int: CommandInteraction, options: string | InteractionReplyOptions | MessagePayload) {
    int.replied || int.deferred ?
    int.followUp(options) :
    int.reply(options);
}

export function replyC(int: ContextMenuInteraction, options: string | InteractionReplyOptions | MessagePayload) {
    int.replied || int.deferred ?
    int.followUp(options) :
    int.reply(options);
}