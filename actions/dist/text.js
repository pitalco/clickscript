export const text = (args) => {
    switch (args.operation) {
        case "concat":
            return args.value;
        case "split":
            return args.value.split(args.delimiter || " ");
        case "replace":
            return args.value.replace(new RegExp(args.delimiter || " ", 'g'), args.replacement || "");
        case "remove":
            return args.startIndex !== undefined && args.endIndex !== undefined
                ? args.value.slice(0, args.startIndex) + args.value.slice(args.endIndex)
                : args.value;
        case "insert":
            return args.startIndex !== undefined
                ? args.value.slice(0, args.startIndex) + (args.insertText || "") + args.value.slice(args.startIndex)
                : args.value + (args.insertText || "");
        case "substring":
            return args.value.substring(args.startIndex || 0, args.endIndex);
        default:
            throw new Error("Invalid operation");
    }
};
