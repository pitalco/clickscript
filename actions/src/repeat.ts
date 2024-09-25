import { Run } from "./types";

/**
 * Repeat argument type for repeat action.
 */
export interface Repeat {
    children: Array<Run>;
};

/**
 * Loop through Runs. We use the children length to determine the loop count.
 * @param args Arguments for the action as a Repeat type.
 */
export const repeat = (args: Repeat): void => {
    for (let i = 0; i < args.children.length; i++) {
        args.children[i].func(args.children[i].args);
    }
};