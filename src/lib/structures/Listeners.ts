import type { Valeriyya } from "#lib/ValeriyyaClient";

export abstract class Listener<O extends Listener.Options = Listener.Options> {

    public name: string;
    public client?: Valeriyya;
    public type: "on" | "once";

    public constructor(options: Listener.Options = {}) {
        this.name = options.name ?? '';
        this.type = options.type ?? 'on';
    }
    
}

export interface ListenerOption {
    name?: string;
    type?: 'on' | 'once';

}

export namespace Listener {
    export type Options = ListenerOption;
}