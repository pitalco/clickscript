import { Run } from "./types.js";
export interface func {
    name: string;
    children: Array<Run>;
    async: boolean;
}
export declare const func: (args: func) => (args: func) => void;
