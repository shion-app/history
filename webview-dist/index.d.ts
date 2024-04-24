export interface Browser {
    name: string;
    last_sync: number;
}
export interface Config {
    browsers: Array<Browser>;
}
export interface History {
    title: string;
    url: string;
    last_visited: number;
}
export declare function getConfig(): Promise<Config>;
export declare function setConfig(config: Config): Promise<unknown>;
export declare function readHistory(list: Array<string>, start: number, end: number): Promise<History[]>;
