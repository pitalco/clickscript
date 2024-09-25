export interface Log {
    message: string[];
    level: "info" | "warn" | "error";
}
export declare const log: (args: Log) => void;
