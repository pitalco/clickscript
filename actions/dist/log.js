export const log = (args) => {
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
