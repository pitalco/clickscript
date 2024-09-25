/**
 * Call argument type for call action.
 */
export interface Call {
    url: string;
    type: "GET" | "POST" | "PUT" | "DELETE";
    data: any;
    headers: Headers;
};

/**
 * Calls an API using fetch.
 * @param args Arguments for the action as a Call type.
 */
export const call = (args: Call): Promise<Response> => {
    switch (args.type) {
        case "GET":
            return fetch(buildGetUrl(args.url, args.data), {
                method: "GET",
                headers: args.headers
            });
        case "POST":
            return fetch(args.url, {
                body: args.data,
                method: "POST",
                headers: args.headers
            });
        case "PUT":
            return fetch(args.url, {
                body: args.data,
                method: "PUT",
                headers: args.headers
            });
        case "DELETE":
            return fetch(args.url, {
                body: args.data,
                method: "DELETE",
                headers: args.headers
            });
        default:
            throw new Error(`Invalid request type: ${args.type}`);
    }
};

const buildGetUrl = (baseUrl: string, queryParams: { [key: string]: any }) => {
    // Convert the query parameters object into a query string
    const queryString = Object.keys(queryParams)
        .map(key => encodeURIComponent(key) + '=' + encodeURIComponent(queryParams[key]))
        .join('&');

    // Append the query string to the base URL
    const urlWithQuery = `${baseUrl}?${queryString}`;
    return urlWithQuery;
}