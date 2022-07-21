<script lang="ts">
  import NavBar from "./components/NavBar.svelte";
  import { TypeStore, LinkStore } from "./components/store";
  import MediaPlayer from "./components/MediaPlayer.svelte";
  // Default theme. ~960B
  import "@vime/core/themes/default.css";

  // Optional light theme (extends default). ~400B
  import "@vime/core/themes/light.css";

  import { VmPlayer, VmVideo, VmFile, defineCustomElements } from "@vime/core";
  import { Player, Youtube } from "@vime/svelte";
  // 1. Manually define them to be as efficient as possible.
  // customElements.define("vm-player", VmPlayer);
  // customElements.define("vm-video", VmVideo);
  // customElements.define("vm-file", VmFile);
  console.log(TypeStore)
  let source: any;
  TypeStore.subscribe((value) => {
    source = value;
  });
  $: youtubelink="";
  LinkStore.subscribe((value) => {
    youtubelink = value;
  });
  // LinkStore.subscribe((value) => {
  //   console.log("NEW VALUE"+ value);
  //   youtubelink = value;
  // });
  // 2. Can't be bothered? Load them all in, may bloat your final bundle size a little.
  defineCustomElements();
</script>

<main>
  <NavBar />
  {#if source == "file"}
    <MediaPlayer />
  {:else}
    <Player controls>
      <Youtube videoId={youtubelink} />
    </Player>
  {/if}
</main>
<svelte:head>
  <!-- Fonts -->
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/icon?family=Material+Icons"
  />
  <link
    rel="stylesheet"
    href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,600,700"
  />

  <!-- Material Typography -->
  <link
    rel="stylesheet"
    href="https://unpkg.com/@material/typography@13.0.0/dist/mdc.typography.css"
  />

  <link
    rel="stylesheet"
    href="https://cdn.jsdelivr.net/npm/@vime/core@^5/themes/default.css"
  />

  <!-- SMUI -->
  <link rel="stylesheet" href="https://unpkg.com/svelte-material-ui/bare.css" />
</svelte:head>
