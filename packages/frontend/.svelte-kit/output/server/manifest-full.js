export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		client: {"start":"_app/immutable/entry/start.d2e980b0.js","app":"_app/immutable/entry/app.61af7ed6.js","imports":["_app/immutable/entry/start.d2e980b0.js","_app/immutable/chunks/index.d339d3f1.js","_app/immutable/chunks/environment.2e1a2b68.js","_app/immutable/entry/app.61af7ed6.js","_app/immutable/chunks/index.7606715b.js","_app/immutable/chunks/index.d339d3f1.js"],"stylesheets":[],"fonts":[]},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		}
	}
}
})();
