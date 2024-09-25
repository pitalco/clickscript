/**
 * Kill argument type for kill action.
 */
export interface Kill {
    message: string;
};

/**
 * Panic and throw an error.
 * @param args Arguments for the action as a Kill type.
 */
export const kill = (args: Kill): void => {
    throw new Error(args.message);
};