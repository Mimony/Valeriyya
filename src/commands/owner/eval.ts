import { AppOptionTypes, defineCommand, IContextInteraction } from "../../lib/util/valeriyya.types";
import { inspect } from "util"

let result = null;

export default defineCommand({
    data: {
        name: "eval",
        type: AppOptionTypes.MESSAGE,
    },
    menu: async (int: IContextInteraction) => {
        if (!["387508127941132308", "206360333881704449"].some(id => int.user.id === id)) {
            return {
                content: `You can't execute this command`,
                ephemeral: true
            }
        }

        try {
            const code = int.options.getMessage("message")!;
            result = await eval(code.content);
        } catch (e) {
            return `Error while evaluating: \`${e}\``;
        }

        if (typeof result !== "string") {
            result = result instanceof Error ? result.stack : inspect(result, { depth: 0 });
        }

        return `\`\`\`typescript\n${result}\`\`\``;
    }
})

