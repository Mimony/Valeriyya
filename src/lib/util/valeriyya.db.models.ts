import type { ActionData } from "./moderation/valeriyya.moderation";

export class Guild {
    public constructor(public id: string) {

    }
}

export interface IGuild {
    id: string;
}

export type IGuildSearch = Pick<IGuild, "id">;

export class Moderation {
    public constructor(public action: ActionData) {

    }
}