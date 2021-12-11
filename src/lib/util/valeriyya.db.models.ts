import type { ActionData } from "./moderation/valeriyya.moderation";

export class Guild {
    public constructor(public id: string) {

    }
}

export class Moderation {
    public constructor(public action: ActionData) {
        
    }
}