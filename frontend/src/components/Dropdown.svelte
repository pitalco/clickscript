<script>
	let isOpen = false;

	function toggleDropdown() {
		isOpen = !isOpen;
	}
</script>

<div class="relative inline-block text-left w-full">
    <button on:click={toggleDropdown} id="dropdownDividerButton" data-dropdown-toggle="dropdownDivider" class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center justify-between dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" type="button">
        Dropdown divider 
        <svg class="w-2.5 h-2.5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 4 4 4-4"/>
        </svg>
    </button>
    
    <!-- Dropdown menu -->
    <div id="dropdownDivider" class={`absolute left-0 right-0 mt-2 z-50 bg-white divide-y divide-gray-100 rounded-lg shadow w-full dark:bg-gray-700 dark:divide-gray-600 ${isOpen ? 'block' : 'hidden'}`}>
        <ul class="py-2 text-sm text-gray-700 dark:text-gray-200" aria-labelledby="dropdownDividerButton">
            <li>
                <a href="#top" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Dashboard</a>
            </li>
            <li>
                <a href="#top" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Settings</a>
            </li>
            <li>
                <a href="#top" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white">Earnings</a>
            </li>
        </ul>
        <div class="py-2">
            <a href="#top" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600 dark:text-gray-200 dark:hover:text-white">Separated link</a>
        </div>
    </div>
</div>
