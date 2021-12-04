import { ActionData, Moderation, Action } from "./valeriyya.moderation"

type BanData = Omit<ActionData, "duration">;

export class Ban extends Moderation {
    public constructor(data: BanData) {
        super(Action.BAN, data)
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