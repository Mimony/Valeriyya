import { ActionData, Moderation, Action } from "./valeriyya.moderation"

type KickData = Omit<ActionData, "duration">;

export class Kick extends Moderation {
    public constructor(data: KickData) {
        super(Action.KICK, data)
    }
    public permissions(): Promise<void> {
        throw new Error("Method not implemented.");
    }
    public execute(): Promise<void> {
        throw new Error("Method not implemented.");
    }
    public db(): Promise<void> {
        throw new Error("Method not implemented.");
    }
}