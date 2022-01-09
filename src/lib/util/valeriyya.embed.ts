import { MessageEmbed, MessageEmbedOptions } from "discord.js";

export class ValeriyyaEmbed extends MessageEmbed {
  public type: "base" | "error";
  public data?: MessageEmbed | MessageEmbedOptions;

  public constructor(data?: MessageEmbed | MessageEmbedOptions, type: "base" | "error" = "base") {
    super();
    this.type = type;
    this.data = data;
    this.type === "base" ? 
    super.setColor("#524264") : 
    super.setColor("#ff0000");
    super.setTimestamp();
  }
}
