/**
 * Log argument type for log action.
 */
export interface Log {
    message: string[];
    level: "info" | "warn" | "error";
};

/**
 * Logs to the console.
 * @param args Arguments for the action as a Log type.
 */
export const log = (args: Log): void => {
    switch (args.level) {
        case "info":
            console.log(...args.message);
            break;
        case "warn":
            console.warn(...args.message);
            break;
        case "error":
            console.error(...args.message);
            break;
        default:
            throw new Error(`Invalid log level: ${args.level}`);
    }
};