/**
 * Set argument type for set action.
 */
export interface Set {
    type: "constant" | "variable";
    name: string;
    value: any;
};

/**
 * Sets a variable or constant globally.
 * @param args Arguments for the action as a Set type.
 */
export const set = (args: Set): void => {
    switch (args.type) {
        case "constant":
            (global as any)[args.name] = args.value;
        case "variable":
            (global as any)[args.name] = args.value;
        default:
            throw new Error("Invalid type");
    }
};