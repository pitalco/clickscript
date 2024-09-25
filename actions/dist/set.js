;
export const set = (args) => {
    switch (args.type) {
        case "constant":
            global[args.name] = args.value;
        case "variable":
            global[args.name] = args.value;
        default:
            throw new Error("Invalid type");
    }
};
