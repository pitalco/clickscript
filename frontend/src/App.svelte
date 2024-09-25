<script lang="ts">
  import { onMount } from 'svelte';
  import { Compile } from '../wailsjs/go/main/App.js';

  let elements = [
    { name: 'div', label: 'Div' },
    { name: 'h2', label: 'Heading 2' },
    { name: 'form', label: 'Form' },
    { name: 'input', label: 'Input' },
    { name: 'button', label: 'Button' }
  ];

  let canvasElements = [];

  async function compile() {
    await Compile();
  }

  function handleDrop(event) {
    event.preventDefault();
    const elementName = event.dataTransfer.getData('text');
    canvasElements = [...canvasElements, { name: elementName }];
    updateJsonFile();
  }

  function handleDragStart(event, elementName) {
    event.dataTransfer.setData('text', elementName);
  }

  async function updateJsonFile() {
    const response = await fetch('/path/to/example.click.json');
    const json = await response.json();

    json.components[0].template.push({
      element: canvasElements[canvasElements.length - 1].name,
      attributes: [],
      children: []
    });

    await fetch('/path/to/example.click.json', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(json)
    });
  }

  onMount(() => {
    const canvas = document.getElementById('canvas');
    canvas.addEventListener('dragover', (event) => event.preventDefault());
    canvas.addEventListener('drop', handleDrop);
  });
</script>

<main class="flex">
  <aside class="w-1/4 p-4 bg-gray-100">
    <h2 class="text-xl font-bold mb-4">Elements</h2>
    <ul>
      {#each elements as element}
        <li
          class="p-2 mb-2 bg-white border rounded cursor-pointer"
          draggable="true"
          on:dragstart={(event) => handleDragStart(event, element.name)}
        >
          {element.label}
        </li>
      {/each}
    </ul>
  </aside>

  <section id="canvas" class="w-3/4 p-4 bg-gray-50 border-l">
    <h2 class="text-xl font-bold mb-4">Canvas</h2>
    {#each canvasElements as element}
      <div class="p-2 mb-2 bg-white border rounded">
        {element.name}
      </div>
    {/each}
  </section>
</main>