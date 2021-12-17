import chalk from "chalk";

export class Logger {
  protected emoji: string;
  protected color: string;
  private errorColor: "#F03A17";
  private errorEmoji: "⛩️";

  constructor() {
    this.emoji = "🌸";
    this.color = "#FF90E0";
    this.errorEmoji = "⛩️";
    this.errorColor = "#F03A17";
  }

  private static getCurrentMemoryHeap() {
    const mem = process.memoryUsage();
    const used = mem.heapUsed / 1000 / 1000;
    const total = mem.heapTotal / 1000 / 1000;

    return `${used.toFixed(2)}/${total.toFixed(2)}MB`;
  }

  public time() {
    return chalk.bold.bgWhite.black(`[${new Date().toLocaleTimeString()}]`);
  }

  public print(log?: any, ...optionalParams: any[]): void {
    console.log(chalk.hex(this.color)(`  ${Logger.getCurrentMemoryHeap()}  ${this.time()} ${this.emoji}  ${log}`), chalk.hex(this.color)(...optionalParams));
  }

  public error(log?: any, ...optionalParams: any[]): void {
    console.log(chalk.hex(this.errorColor)(`  ${Logger.getCurrentMemoryHeap()}  ${this.time()} ${this.errorEmoji}  ${log}`), chalk.hex(this.errorColor)(...optionalParams));
  }
}

