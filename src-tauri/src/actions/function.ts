import { Run } from "./types.js";

export interface func {
    name: string;
    children: Array<Run>;
    async: boolean;
};

export const func = (args: func) => {
    if (args.async) {
        (global as any)[args.name] = async function() {
            for (let i = 0; i < args.children.length; i++) {
                await args.children[i].func(args.children[i].args);
            }
        }
    }
    return function(args: func) {
        (global as any)[args.name] = function() {
            for (let i = 0; i < args.children.length; i++) {
                args.children[i].func(args.children[i].args);
            }
        }
    };
};