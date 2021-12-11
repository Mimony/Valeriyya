// Information Commands
import user from "../commands/info/user";

// Moderation Commands
import ban from "../commands/moderation/ban";
import kick from "../commands/moderation/kick"

export const Commands = () => {
    return [
        user,
        ban,
        kick
    ];
}