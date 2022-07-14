<script>
  export let opend = false;
  import { open } from "@tauri-apps/api/dialog";
  import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	
  // Open a selection dialog for image files
  async function selectTime() {
    const selected = await open({
      multiple: true,
    });
    console.log("ok")
    if (Array.isArray(selected)) {
      // user selected multiple files
      function sayHello() {
        dispatch('gotvideo', {
          selected: selected
        });
      }
      sayHello();
    } else if (selected === null) {
      // user cancelled the selection
      console.log("ok2")
    } else {
      console.log(selected)
      // user selected a single file
    }
  }
</script>

<aside
  class="absolute w-full h-full bg-gray-200 border-r-2 shadow-lg"
  class:opend
>
  <nav class="p-12 text-xl">
    <p
      class="block"
      on:click={() => {
        selectTime();
      }}
    >
      Open File
    </p>
    <p class="block">YouTube</p>
  </nav>
</aside>

<style>
  aside {
    left: -100%;
    transition: left 0.3s ease-in-out;
  }

  .opend {
    left: 0;
  }
</style>
