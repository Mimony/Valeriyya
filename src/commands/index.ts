// Information Commands
import user from "../commands/info/user";

// Moderation Commands
import ban from "../commands/moderation/ban";
import kick from "../commands/moderation/kick";
import reason from "../commands/moderation/reason";
import cases from "../commands/moderation/cases/cases";

// Setting Commands
import settings from "../commands/settings/settings";

export const Commands = () => {
    return [
        user,
        ban,
        kick,
        reason,
        settings,
        cases
    ];
}