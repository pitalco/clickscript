;
export const repeat = (args) => {
    for (let i = 0; i < args.children.length; i++) {
        args.children[i].func(args.children[i].args);
    }
};
