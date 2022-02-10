import type { Valeriyya } from "../../valeriyya.client";
import { PrismaClient } from "@prisma/client";

export class ValeriyyaDB {
  public client: Valeriyya;
  public dbClient: PrismaClient;

  public constructor(client: Valeriyya) {
    this.client = client;
    this.dbClient = new PrismaClient();
  }

  public async on() {
    return this.dbClient.$connect()
  }

}
