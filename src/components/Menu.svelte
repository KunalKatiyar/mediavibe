<script lang="ts">
  export let opend = false;
  import { open } from "@tauri-apps/api/dialog";
  import { createEventDispatcher } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import {ItemsStore} from "./store";
  console.log(ItemsStore[0])

  // Open a selection dialog for image files
  async function selectTime() {
    const selected = await open({
      multiple: true,
    });
    console.log("ok");
    if (Array.isArray(selected)) {
      // user selected multiple files
      const assetUrl = convertFileSrc(selected[0]);
      $ItemsStore[0].selected = assetUrl;
      console.log($ItemsStore[0].selected)
    } else if (selected === null) {
      // user cancelled the selection
      console.log("ok2");
    } else {
      console.log(selected);
      // user selected a single file
    }
  }
</script>

<nav class="hidden text-gray-500 uppercase text-bold sm:block">
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
