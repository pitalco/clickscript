/**
 * Concat argument type for concat operation.
 */
export interface ConcatArgs {
    value: string;
    operationText: string;
}

/**
 * Concatenates two strings.
 * @param args Arguments for the concat operation.
 */
export const concat = (args: ConcatArgs): string => {
    return args.value + args.operationText;
};

/**
 * Split argument type for split operation.
 */
export interface SplitArgs {
    value: string;
    delimiter?: string;
}

/**
 * Splits a string into an array of substrings.
 * @param args Arguments for the split operation.
 */
export const split = (args: SplitArgs): string[] => {
    return args.value.split(args.delimiter || " ");
};

/**
 * Replace argument type for replace operation.
 */
export interface ReplaceArgs {
    value: string;
    delimiter?: string;
    replacement?: string;
}

/**
 * Replaces all occurrences of a substring in a string.
 * @param args Arguments for the replace operation.
 */
export const replace = (args: ReplaceArgs): string => {
    return args.value.replace(new RegExp(args.delimiter || " ", 'g'), args.replacement || "");
};

/**
 * Remove argument type for remove operation.
 */
export interface RemoveArgs {
    value: string;
    startIndex?: number;
    endIndex?: number;
}

/**
 * Removes a portion of a string based on start and end indices.
 * @param args Arguments for the remove operation.
 */
export const remove = (args: RemoveArgs): string => {
    return args.startIndex !== undefined && args.endIndex !== undefined
        ? args.value.slice(0, args.startIndex) + args.value.slice(args.endIndex)
        : args.value;
};

/**
 * Insert argument type for insert operation.
 */
export interface InsertArgs {
    value: string;
    insertText?: string;
    startIndex?: number;
}

/**
 * Inserts a substring into a string at a specified index.
 * @param args Arguments for the insert operation.
 */
export const insert = (args: InsertArgs): string => {
    return args.startIndex !== undefined
        ? args.value.slice(0, args.startIndex) + (args.insertText || "") + args.value.slice(args.startIndex)
        : args.value + (args.insertText || "");
};

/**
 * Substring argument type for substring operation.
 */
export interface SubstringArgs {
    value: string;
    startIndex?: number;
    endIndex?: number;
}

/**
 * Extracts a portion of a string based on start and end indices.
 * @param args Arguments for the substring operation.
 */
export const substring = (args: SubstringArgs): string => {
    return args.value.slice(args.startIndex || 0, args.endIndex);
};