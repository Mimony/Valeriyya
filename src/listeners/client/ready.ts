import { Listener } from "#lib/structures/Listeners";

export default class extends Listener {
  public constructor() {
    super({
      name: "ready",
    });
  }

  public execute() {
    this.client?.fragments.loadCommands();
    const cmds = this.client?.commands!;

    const keys = Object.keys(cmds);
    for (let i = 0; i < keys.length; i++) {
      cmds.set(keys[i], cmds.get(keys[i])!);
    }

    console.log(`${this.client?.user?.tag} is ready!`)
  }
}
