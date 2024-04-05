import { c as create_ssr_component, d as add_attribute, e as escape, v as validate_component } from "../../chunks/index.js";
import "@tauri-apps/api/tauri";
const Greet = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let path = "/root/clickscript/example.json";
  let script = "";
  return `<div class="container mx-auto flex flex-col bg-white border shadow-sm rounded-xl p-4 md:p-5 dark:bg-gray-800 dark:border-gray-700 dark:shadow-slate-700/[.7] dark:text-gray-400"><div class="grid grid-cols-4 gap-4"><input class="col-span-3 py-3 px-4 block w-full border-gray-200 rounded-md text-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400" placeholder="Enter Path"${add_attribute("value", path, 0)}>
    <button class="py-[.688rem] px-4 inline-flex justify-center items-center gap-2 rounded-md border-2 border-gray-200 font-semibold text-blue-500 hover:text-white hover:bg-blue-500 hover:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all text-sm dark:border-gray-700 dark:hover:border-blue-500">Compile
    </button></div>
  <p>${escape(script)}</p></div>`;
});
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `${validate_component(Greet, "Greet").$$render($$result, {}, {}, {})}`;
});
export {
  Page as default
};
