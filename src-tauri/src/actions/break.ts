export interface Kill {
    message: string;
};

export const kill = (args: Kill) => {
    throw new Error(args.message);
};