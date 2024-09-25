export interface Call {
    url: string;
    type: "GET" | "POST" | "PUT" | "DELETE";
    data: any;
    headers: Headers;
}
export declare const call: (args: Call) => Promise<Response>;
