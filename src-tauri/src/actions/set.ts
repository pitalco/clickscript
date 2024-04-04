export interface Set {
    type: "constant" | "variable";
    name: string;
    value: any;
};

export const set = (args: Set) => {
    switch (args.type) {
        case "constant":
            (global as any)[args.name] = args.value;
        case "variable":
            (global as any)[args.name] = args.value;
        default:
            throw new Error("Invalid type");
    }
};