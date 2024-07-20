import { Run } from "./types.ts";

export interface Repeat {
    children: Array<Run>;
};

export const repeat = (args: Repeat) => {
    for (let i = 0; i < args.children.length; i++) {
        args.children[i].func(args.children[i].args);
    }
};