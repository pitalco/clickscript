export interface Set {
    type: "constant" | "variable";
    name: string;
    value: any;
}
export declare const set: (args: Set) => never;
