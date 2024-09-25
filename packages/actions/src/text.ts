/**
 * Text argument type for text action.
 */
export interface Text {
    operation: "concat" | "split" | "replace" | "remove" | "insert" | "substring";
    value: string;
    delimiter?: string;
    replacement?: string;
    insertText?: string;
    startIndex?: number;
    endIndex?: number;
}

/**
 * Perform various text operations on a text.
 * @param args Arguments for the action as a Text type.
 */
export const text = (args: Text): string | string[] => {
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