<script>import "./app.css";
import { onMount } from 'svelte';
import { RenderHTML } from '../wailsjs/go/main/App.js';
import interact from 'interactjs';
import appJson from '../../example.click.json';

$: console.log('renderedHTML', renderedHTML);

let elements = [
  { name: 'div', label: 'Div', icon: '□' },
  { name: 'h2', label: 'Heading 2', icon: 'H2' },
  { name: 'form', label: 'Form', icon: '⎚' },
  { name: 'input', label: 'Input', icon: '⌨' },
  { name: 'button', label: 'Button', icon: '⏵' }
];

let json = appJson;
let renderedHTML = '';


</script>

<button data-drawer-target="sidebar-double" data-drawer-toggle="sidebar-double" aria-controls="sidebar-double" type="button" class="inline-flex items-center p-2 mt-2 ml-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600">
  <span class="sr-only">Open sidebar</span>
  <svg class="w-6 h-6" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
     <path clip-rule="evenodd" fill-rule="evenodd" d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"></path>
  </svg>
</button>

<aside id="sidebar-double" class="flex z-40 fixed top-0 left-0 h-full transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
 <div class="overflow-y-auto z-30 py-5 px-3 w-16 h-full bg-white border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700">
     <a href="#">
         <img src="https://flowbite.com/docs/images/logo.svg" class="pl-2 mb-6 h-7" alt="Flowbite Logo" />
     </a>
     <ul class="space-y-2">
         <li>
             <a href="#" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                 <svg aria-hidden="true" class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"></path></svg>
             </a>
         </li>
         <li>
             <a href="#" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                 <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path></svg>
             </a>
         </li>
         <li>
             <a href="#" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                 <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z"></path><path d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z"></path></svg>
             </a>
         </li>
         <li>
             <a href="#" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                 <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M8.707 7.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l2-2a1 1 0 00-1.414-1.414L11 7.586V3a1 1 0 10-2 0v4.586l-.293-.293z"></path><path d="M3 5a2 2 0 012-2h1a1 1 0 010 2H5v7h2l1 2h4l1-2h2V5h-1a1 1 0 110-2h1a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V5z"></path></svg>
             </a>
         </li>
         <li>
             <a href="#" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                 <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd"></path></svg>
             </a>
         </li>
     </ul>
 </div>
 <div id="secondary-sidenav" class="overflow-y-auto relative py-5 px-3 w-64 h-full bg-white border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700">
     <ul class="space-y-2">
        <div class="flex justify-between items-center pt-3 pl-2 mt-5 mb-2 text-sm font-medium text-gray-500 uppercase dark:text-gray-400">
            <h3>Groups</h3>
        </div>
         <li>
             <a href="#" class="cursor-pointer flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group">
                 <span class="ml-3">Group</span>
             </a>
         </li>
         <li>
          <a href="#" class="cursor-pointer flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group">
              <span class="ml-3">Repeating Group</span>
          </a>
        </li>
     </ul>
     <div class="flex absolute right-2 bottom-2 z-20 justify-end w-full bg-white dark:bg-gray-800">
         <button on:click={() => document.getElementById('secondary-sidenav').style.display = 'none'} id="show-secondary-sidenav-button" aria-controls="secondary-sidenav" type="button" class="inline-flex p-2 text-gray-500 rounded-full cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:hover:bg-gray-700 dark:hover:text-white">
             <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path></svg>
         </button>
     </div>
 </div>
 <button on:click={() => document.getElementById('secondary-sidenav').style.display = 'block'} id="hide-secondary-sidenav-button" aria-controls="secondary-sidenav" type="button" class="inline-flex absolute bottom-2 left-20 p-2 text-gray-500 rounded-full cursor-pointer dark:text-gray-400 hover:text-gray-900 hover:bg-gray-200 dark:hover:bg-gray-700 dark:bg-gray-900 dark:hover:text-white bg-white">
     <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"></path></svg>
 </button>
</aside>