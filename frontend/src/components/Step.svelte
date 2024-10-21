<script lang="ts">
	export let title: string;
	export let description: string;

    export let onClick: () => void;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="min-w-60 mt-3">
    <div on:click={onClick}>
        <a href="#top" class="block max-w-sm p-4 bg-white border border-gray-200 rounded-lg shadow hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-700 dark:hover:bg-gray-700">
        <h5 class="mb-2 text-lg font-bold tracking-tight text-gray-900 dark:text-white">{title}</h5>
            <p class="font-normal text-gray-700 dark:text-gray-400 text-sm">{description}</p>
        </a>
    </div>
    <svg class="w-6 h-6 text-gray-800 dark:text-white flex justify-center items-center w-full mt-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19V5m0 14-4-4m4 4 4-4"/>
      </svg>    
</div>