<script>
  export let opend = false;
  import { open } from "@tauri-apps/api/dialog";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  // Open a selection dialog for image files
  async function selectTime() {
    const selected = await open({
      multiple: true,
    });
    console.log("ok");
    if (Array.isArray(selected)) {
      // user selected multiple files
      function sayHello() {
        dispatch("gotvideo", {
          selected: selected,
        });
      }
      sayHello();
    } else if (selected === null) {
      // user cancelled the selection
      console.log("ok2");
    } else {
      console.log(selected);
      // user selected a single file
    }
  }

  async function selectYoutube() {
    console.log("start");
    let data = {
      api_key: "AIzaSyBlvGk41Anjer4QegVzE7WXoJE0Bx6lNIg",
      part: "snippet",
      maxResults: 1,
      q: "9 巻発売記念スペシャルPV",
      type: "video",
    }
    let api_key = "AIzaSyBlvGk41Anjer4QegVzE7WXoJE0Bx6lNIg";
    let url = new URL(
      "https://www.googleapis.com/youtube/v3/search");
    for (let k in data){
      url.searchParams.append(k, data[k]);
    }
    let response = await fetch(url);
    console.log(response);
  }
</script>

<aside
  class="absolute w-full h-full bg-gray-200 border-r-2 shadow-lg"
  class:opend
>
  <nav class="p-14 text-xl">
    <p
      class="block"
      on:click={() => {
        selectTime();
      }}
    >
      Open File
    </p>
    <p
      class="block"
      on:click={() => {
        selectYoutube();
      }}
    >
      YouTube
    </p>
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
