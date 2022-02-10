// Information Commands
import user from "../commands/info/user";

// Moderation Commands
import ban from "../commands/moderation/ban";
import kick from "../commands/moderation/kick";
import mute from "../commands/moderation/mute"
import reason from "../commands/moderation/reason";
import cases from "../commands/moderation/cases/cases";
import history from "../commands/moderation/history";

// Setting Commands
import settings from "../commands/settings/settings";

// Owner Command
import eval from "../commands/owner/eval"

export const Commands = () => {
    return [
        user,
        ban,
        kick,
        mute,
        reason,
        settings,
        cases,
        eval,
        history
    ];
}