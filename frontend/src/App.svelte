<script lang="ts">
import "./app.css";
import interact from 'interactjs';
import appJson from '../../example.click.json';
import Step from './components/Step.svelte';
import Popup from './components/Popup.svelte';

let isOpen: boolean = false;
</script>

<main class="w-full h-screen flex overflow-hidden bg-white">      
      <aside id="sidebar-double" class="flex z-40 fixed top-0 left-0 h-full transition-transform -translate-x-full sm:translate-x-0" aria-label="Sidebar">
       <div class="overflow-y-auto z-30 py-5 px-3 w-16 h-full bg-white border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700">
           <a href="#top">
               <img src="https://flowbite.com/docs/images/logo.svg" class="pl-2 mb-6 h-7" alt="Flowbite Logo" />
           </a>
           <ul class="space-y-2">
               <li>
                   <a href="#top" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                       <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path></svg>
                   </a>
               </li>
               <li>
                   <a href="#top" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                       <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z"></path><path d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z"></path></svg>
                   </a>
               </li>
               <li>
                   <a href="#top" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                       <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M8.707 7.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l2-2a1 1 0 00-1.414-1.414L11 7.586V3a1 1 0 10-2 0v4.586l-.293.293a1 1 0 001.414-1.414l-7-7z"></path><path d="M3 5a2 2 0 012-2h1a1 1 0 010 2H5v7h2l1 2h4l1-2h2V5h-1a1 1 0 110-2h1a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V5z"></path></svg>
                   </a>
               </li>
               <li>
                   <a href="#top" class="flex items-center p-2 text-gray-400 rounded-lg transition duration-75 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                       <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd"></path></svg>
                   </a>
               </li>
           </ul>
       </div>
      </aside>
      <div class="flex justify-center items-center w-full h-full p-10">
        <div class="flex-col bg-white overflow-auto justify-center items-baselinep-5">
            <Step onClick={() => isOpen = true} on:x-clicked={() => isOpen = false} title="Step 1" description="Create a new Mission" />  
            <Step onClick={() => isOpen = true} on:x-clicked={() => isOpen = false} title="Step 1" description="Update a Mission" />  
            <Step onClick={() => isOpen = true} on:x-clicked={() => isOpen = false} title="Step 3" description="Call API DefundServer" />  
            <Step onClick={() => isOpen = true} on:x-clicked={() => isOpen = false} title="Step 4" description="Set State on GroupFour" />    
        </div>
        <Popup isOpen={isOpen} on:x-clicked={() => isOpen = false} />
      </div>
</main>