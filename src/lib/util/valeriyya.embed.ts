import {MessageEmbed} from "discord.js";

export class ValeriyyaEmbed extends MessageEmbed {
    public type: "base" | "error";
    public constructor(type: "base" | "error" = "base") {
        super();
        this.type = type;
        this.type === "base" ?
            super.setColor("#524264") :
            super.setColor("#ff0000");
        super.setTimestamp();
    }

}