import { Run } from "./types.ts";

/**
 * func argument type for func action.
 */
export interface func {
    name: string;
    children: Array<Run>;
    async: boolean;
};

/**
 * Sets a function in global to be run another time.
 * @param args Arguments for the action as a func type.
 */
export const func = (args: func): (args: func) => void => {
    if (args.async) {
        (globalThis as any)[args.name] = async function() {
            for (let i = 0; i < args.children.length; i++) {
                await args.children[i].func(args.children[i].args);
            }
        }
    }
    return function(args: func) {
        (globalThis as any)[args.name] = function() {
            for (let i = 0; i < args.children.length; i++) {
                args.children[i].func(args.children[i].args);
            }
        }
    };
};