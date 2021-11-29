import type { Valeriyya } from "#lib/ValeriyyaClient";
import { readdirSync } from "fs";
import { join } from "path";

export class Fragments {
  public client: Valeriyya;
  public path: { commands: string; listeners: string };

  public constructor(
    client: Valeriyya,
    path: { commands: string; listeners: string } = {
      commands: join(__dirname, "..", "commands"),
      listeners: join(__dirname, "..", "listeners"),
    }
  ) {
    this.client = client;
    this.path = path;
  }

  public async loadListeners(path: string = this.path.listeners) {
    const categories = readdirSync(path);

    for (let i = 0; i < categories.length; i++) {
      const category = categories[i];
      const listeners = readdirSync(`${path}/${category}`).filter((f) => f.endsWith(".js"));

      for (let i = 0; i < listeners.length; i++) {
        const listener = (await import(`${path}/${category}/${listeners[i]}`)).default;
        listener.client = this.client;

        this.client[listener.type as "on" | "once"](listener.name, listener.execute);
      }
    }
  }

  public async loadCommands(path: string = this.path.commands) {
    const categories = readdirSync(path);

    for (let i = 0; i < categories.length; i++) {
      const category = categories[i];
      const commands = readdirSync(`${path}/${category}`).filter((f) => f.endsWith(".js"));

      for (let i = 0; i < commands.length; i++) {
        const command = (await import(`${path}/${category}/${commands[i]}`)).default;
        this.client.commands.set(command.name, command);
      }
    }
  }
}
