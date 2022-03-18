import type { Valeriyya } from "../../valeriyya.client";
import { DataSource } from "typeorm";
import { GuildDb } from "./entities/Guild";

export class ValeriyyaDB {
  public client: Valeriyya;

  public constructor(client: Valeriyya) {
    this.client = client;
  }

  public async on() {
    const connection = new DataSource({
      url: "mongodb+srv://Client:MomsSpaghetti@cluster0.i1oux.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
      type: "mongodb",
      useUnifiedTopology: true,
      entities: [GuildDb],
    })

    await connection.initialize();

    console.log(connection.getRepository("guild"))
    if(connection.isInitialized) this.client.logger.print`The connection to the database has been established.`
    else this.client.logger.print`The database connection has failed!`
  }

}
