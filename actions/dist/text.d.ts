export interface Text {
    operation: "concat" | "split" | "replace" | "remove" | "insert" | "substring";
    value: string;
    delimiter?: string;
    replacement?: string;
    insertText?: string;
    startIndex?: number;
    endIndex?: number;
}
export declare const text: (args: Text) => string | string[];
