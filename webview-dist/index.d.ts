interface Browser {
    name: string;
    last_sync: number;
}
interface Config {
    browsers: Array<Browser>;
}
export declare function getConfig(): Promise<Config>;
export {};
