// Information Commands
import user from "./info/user";

// Moderation Commands
import ban from "./moderation/ban";
import kick from "./moderation/kick";
import mute from "./moderation/mute"
import reason from "./moderation/reason";
import cases from "./moderation/cases/cases";
import history from "./moderation/history";

// Setting Commands
import settings from "./settings/settings";

// Owner Command
import eval from "./owner/eval"

// Music Commands
import play from "./music/play"
import nowplaying from "./music/nowplaying";
import disconnect from "./music/disconnect";
import pause from "./music/pause";
import resume from "./music/resume";
import skip from "./music/skip";
import queue from "./music/queue";
import remove from "./music/remove";
import loop from "./music/loop";

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
        history,
        play,
        skip,
        nowplaying,
        disconnect,
        pause,
        resume,
        remove,
        queue,
        loop
    ];
}
