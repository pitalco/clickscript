import { Run } from "./types.js";
export interface Repeat {
    children: Array<Run>;
}
export declare const repeat: (args: Repeat) => void;
