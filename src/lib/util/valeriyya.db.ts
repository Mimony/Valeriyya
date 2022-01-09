import type { Valeriyya } from "../valeriyya.client";
import { type Connection, createConnection } from "typeorm";
import { GuildEntity } from "./valeriyya.db.models";

export class ValeriyyaDB {
  public client: Valeriyya;

  public constructor(client: Valeriyya) {
    this.client = client;
  }

  public async on(uri: string): Promise<Connection> {
    const connection = await createConnection({
      type: "mongodb",
      url: uri,
      useNewUrlParser: true,
      useUnifiedTopology: true,
      synchronize: false,
      entities: [GuildEntity],
    });
    if (connection.isConnected) this.client.logger.print(`Database is connected.`);
    else this.client.logger.error(`Database connection has failed.`);
    return connection;
  }
}
