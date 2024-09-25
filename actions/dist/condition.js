;
export const condition = (args) => {
    if (args.statement.func(args.statement.args)) {
        for (let i = 0; i < args.children.length; i++) {
            args.children[i].func(args.children[i].args);
        }
    }
};
