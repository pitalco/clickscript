<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import interact from 'interactjs';

	export let isOpen: boolean = false;

	const dispatch = createEventDispatcher();

	function onCloseClick() {
		dispatch('x-clicked');
	}

	let popupElement: HTMLElement;

	let headerText = "Static modal";
	let isHeaderEditing = false;

	function toggleHeaderEdit() {
		isHeaderEditing = !isHeaderEditing;
	}

	function handleHeaderKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			isHeaderEditing = false;
		}
	}

	onMount(() => {
		interact(popupElement).draggable({
			allowFrom: '.draggable-area',
			listeners: {
				move(event) {
					const target = event.target;
					const x = (parseFloat(target.getAttribute('data-x')) || 0) + event.dx;
					const y = (parseFloat(target.getAttribute('data-y')) || 0) + event.dy;

					target.style.transform = `translate(${x}px, ${y}px)`;
					target.setAttribute('data-x', x);
					target.setAttribute('data-y', y);
				},
			},
		});
	});
</script>

<!-- Main modal -->
<div
	bind:this={popupElement}
	id="static-modal"
	data-modal-backdrop="static"
	tabindex="-1"
	class={`${isOpen ? 'block' : 'hidden'} overflow-y-auto overflow-x-hidden fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 w-full max-w-sm`}
>
	<div class="relative w-full max-h-full">
		<!-- Modal content -->
		<div class="relative bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
			<!-- Modal header -->
			<div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600 draggable-area">
				<div class="flex-grow overflow-hidden">
					{#if isHeaderEditing}
						<!-- svelte-ignore a11y-autofocus -->
						<input
							type="text"
							bind:value={headerText}
							on:blur={toggleHeaderEdit}
							on:keydown={handleHeaderKeydown}
							class="text-xl font-bold tracking-tight text-gray-900 dark:text-white bg-transparent w-full focus:outline-none focus:ring-0 border-0"
							autofocus
						/>
					{:else}
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<h3 
							on:click={toggleHeaderEdit}
							class="text-xl font-bold tracking-tight text-gray-900 dark:text-white cursor-text text-left truncate"
						>
							{headerText}
						</h3>
					{/if}
				</div>
				<button on:click={onCloseClick} type="button" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white cursor-pointer" data-modal-hide="static-modal">
					<svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
						<path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
					</svg>
					<span class="sr-only">Close modal</span>
				</button>
			</div>
			<!-- Modal body -->
			<div class="p-4 md:p-5 space-y-4 draggable-area">
				<div>
					<label for="input1" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Input 1</label>
					<input type="text" id="input1" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter text here">
				</div>
				<div>
					<label for="input2" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Input 2</label>
					<input type="text" id="input2" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Enter text here">
				</div>
			</div>
			<!-- Modal footer -->
			<div class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600 draggable-area default-cursor">
				<button data-modal-hide="static-modal" type="button" class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">I accept</button>
				<button data-modal-hide="static-modal" type="button" class="py-2.5 px-5 ms-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700">Decline</button>
			</div>
		</div>
	</div>
</div>

<style>
	.draggable-area {
		cursor: move;
	}
	.draggable-area.default-cursor {
		cursor: default;
	}
	.draggable-area input,
	.draggable-area button {
		cursor: default;
	}
	.cursor-pointer {
		cursor: pointer;
	}
	input:focus {
		box-shadow: none;
	}
	input, h3 {
		text-align: left;
	}
</style>
