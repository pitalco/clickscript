<script>
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    let name = '';
    let type = '';
    let value = '';

    const handleSubmit = () => {
        dispatch('submit', { name, type, value });
        name = type = value = '';
    };
</script>

<div class="fixed z-10 inset-0 overflow-y-auto">
    <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
        <div class="fixed inset-0 transition-opacity" aria-hidden="true">
        <div class="absolute inset-0 bg-gray-500 opacity-75"></div>
        </div>
        <span class="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">&#8203;</span>
        <div class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
        <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
            <div class="sm:flex sm:items-start">
            <div class="mt-3 text-center sm:mt-0 sm:text-left">
                <h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-title">
                Fill the data
                </h3>
                <div class="mt-2">
                <input bind:value={name} class="w-full px-2 py-1 border rounded" type="text" placeholder="Name" />
                <input bind:value={type} class="w-full px-2 py-1 border rounded mt-2" type="text" placeholder="Type" />
                <input bind:value={value} class="w-full px-2 py-1 border rounded mt-2" type="text" placeholder="Value" />
                </div>
            </div>
            </div>
        </div>
        <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
            <button on:click={handleSubmit} type="button" class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-green-600 text-base font-medium text-white hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 sm:ml-3 sm:w-auto sm:text-sm">
            Submit
            </button>
        </div>
        </div>
    </div>
</div> 

<style>
</style>