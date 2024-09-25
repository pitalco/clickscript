import { Run } from "./types.js";
export interface Condition {
    statement: Run;
    children: Array<Run>;
}
export declare const condition: (args: Condition) => void;
