<script>
  export const opend = false;
  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { ItemsStore, LinkStore, TypeStore, TitleStore } from "./store";

  async function selectTime() {
    const selected = await open({
      multiple: true,
    });
    console.log("ok");
    if (Array.isArray(selected)) {
      const assetUrl = convertFileSrc(selected[0]);
      ItemsStore.update(() => assetUrl);
      TypeStore.update(() => "file");
    } else if (selected === null) {
      console.log("ok2");
    } else {
      console.log(selected);
    }
  }

  async function selectYoutube() {
    console.log("start");
    let title;
    TitleStore.subscribe((value) => {
      title = value;
    });
    let data = {
      key: "AIzaSyBlvGk41Anjer4QegVzE7WXoJE0Bx6lNIg",
      part: "snippet",
      maxResults: 1,
      q: title,
      type: "video",
    }
    let url = new URL(
      "https://www.googleapis.com/youtube/v3/search");
    for (let k in data){
      url.searchParams.append(k, data[k]);
    }
    let response = await fetch(url);
    
    let obj = await response.json();
    console.log(obj.items[0].id.videoId);
    TypeStore.update(() => "youtube");
    LinkStore.update(()=> obj.items[0].id.videoId);
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
  <p
    class="block"
    on:click={() => {
      selectYoutube();
    }}
  >
    YouTube
  </p>
</nav>
