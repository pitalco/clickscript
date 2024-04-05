

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.2785dbe1.js","_app/immutable/chunks/index.7606715b.js","_app/immutable/chunks/environment.2e1a2b68.js","_app/immutable/chunks/index.d339d3f1.js"];
export const stylesheets = [];
export const fonts = [];
