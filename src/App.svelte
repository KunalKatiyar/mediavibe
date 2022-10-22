<script lang="ts">
  import NavBar from "./components/NavBar.svelte";
  import { TypeStore, LinkStore } from "./components/store";
  import MediaPlayer from "./components/MediaPlayer.svelte";
  import "@vime/core/themes/default.css";
  import "@vime/core/themes/light.css";
  import { Player, Youtube } from "@vime/svelte";

  console.log(TypeStore)
  let source: any;
  TypeStore.subscribe((value) => {
    source = value;
  });
  $: youtubelink="";
  LinkStore.subscribe((value) => {
    
    youtubelink = value;
    console.log("youtube link " + youtubelink);
  });
  
</script>

<main>
  <NavBar />
  {#if source == "file"}
    <MediaPlayer />
  {:else}
    <Player controls>
      {#key youtubelink}
        <Youtube videoId={youtubelink} />
      {/key}
    </Player>
  {/if}
</main>
<svelte:head>
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/icon?family=Material+Icons"
  />
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,600,700"
  />
  <link
    rel="stylesheet"
    href="https://unpkg.com/@material/typography@13.0.0/dist/mdc.typography.css"
  />

  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/@vime/core@^5/themes/default.css"
  />
  <link rel="stylesheet" href="https://unpkg.com/svelte-material-ui/bare.css" />
</svelte:head>
