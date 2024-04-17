interface Browser {
    name: string;
    last_sync: number;
}
interface Config {
    browsers: Array<Browser>;
}
interface History {
    title: string;
    url: string;
    last_visited: number;
}
export declare function getConfig(): Promise<Config>;
export declare function readHistory(list: Array<string>, start: number, end: number): Promise<History[]>;
export {};
