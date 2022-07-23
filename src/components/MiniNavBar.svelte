<script>
  import Logo from "./Logo.svelte";
  import Hamburger from "./Hamburger.svelte";
  import Menu from "./Menu.svelte";
  import Button from '@smui/button';
  import { TypeStore, TitleStore, LinkStore } from "./store";
  export let sidebar = false;
  export let searchTerm;
  let type;

  TypeStore.subscribe((value) => {
    type = value;
  });

  async function SearchTime() {
    TitleStore.update(() => searchTerm);
    await selectYoutube();
  }

  async function selectYoutube() {
    console.log("start");
    let data = {
      key: "AIzaSyCvHNL-WTfpdpmbK2mk9I0ahxvedEPHjyc",
      part: "snippet",
      maxResults: 1,
      q: searchTerm,
      type: "video",
    };
    let url = new URL("https://www.googleapis.com/youtube/v3/search");
    for (let k in data) {
      url.searchParams.append(k, data[k]);
    }
    let response = await fetch(url);

    let obj = await response.json();
    console.log(obj.items[0].id.videoId);
    TypeStore.update(() => "file");
    TypeStore.update(() => "youtube");
    LinkStore.update(() => obj.items[0].id.videoId);
  }
</script>

<header
  class="flex justify-between bg-gray-200 p-2 items-center text-gray-600 border-b-2"
>
  <nav class="flex">
    <Hamburger bind:opend={sidebar} />
    <Logo />
    {#if type == "youtube"}
      <div class="flex">
        <div id="search-input-cont">
          <input
            type="text"
            id="search-field"
            placeholder="Enter Youtube Search"
            autocomplete="off"
            bind:value={searchTerm}
            on:input
          />
        </div>
        <div>
          <Button
          style="vertical-align: middle;"
            on:click={() => {
              SearchTime();
            }}>Search</Button
          >
        </div>
      </div>
    {/if}
  </nav>

  <Menu />
</header>

<style>
  #search-input-cont {
    width: 100%;
    display: flex;
    align-items: center;
  }

  #search-field {
    width: 100%;
    font-size: 0.8rem;
    border: 1px solid gray;
    border-radius: 4px;
    padding: 8px;
    margin: 0 10px 0;
  }
</style>
