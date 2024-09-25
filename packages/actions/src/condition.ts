import { Run } from "./types.ts";

/**
 * Condition argument type for condition action.
 */
export interface Condition {
    statement: Run;
    children: Array<Run>;
};

/**
 * Uses a Clickscript function to determine if a condition passes and runs the Run if it passes.
 * @param args Arguments for the action as a Condition type.
 */
export const condition = (args: Condition): void => {
    if (args.statement.func(args.statement.args)) {
        for (let i = 0; i < args.children.length; i++) {
            args.children[i].func(args.children[i].args);
        }
    }
};