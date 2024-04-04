import { Run } from "./types.ts";

export interface Condition {
    statement: Run;
    children: Array<Run>;
};

export const condition = (args: Condition) => {
    if (args.statement.func(args.statement.args)) {
        for (let i = 0; i < args.children.length; i++) {
            args.children[i].func(args.children[i].args);
        }
    }
};