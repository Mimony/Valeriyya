export class Guild {
    public id: string;
    public cases?: Cases[];
    public cases_number: number | 0;

    public constructor({id, cases, cases_numbers}: Guild_Ctor) {
        this.id = id;
        this.cases = cases;
        this.cases_number = cases_numbers;
    }
}

interface Guild_Ctor {
    id: string;
    cases?: Cases[];
    cases_numbers: number | 0;
}

export interface Cases {
    int?: string;
    id: number;
    type: "ban" | "kick" | "mute" | "unban" | "unmute"
    guildId: string;
    staffId: string;
    targetId: string;
    date: Date;
    reason: string | "No reason!";
    duration: number | 0;
}

export interface IGuild {
    id: string;
    cases?: Cases[];
    cases_number?: number;
}

export type IGuildSearch = Pick<IGuild, "id">;