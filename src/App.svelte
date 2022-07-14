<script lang="ts">
  import NavBar from "./components/NavBar.svelte";
  import MediaPlayer from "./components/MediaPlayer.svelte";
  // Default theme. ~960B
  import "@vime/core/themes/default.css";

  // Optional light theme (extends default). ~400B
  import "@vime/core/themes/light.css";

  import { VmPlayer, VmVideo, VmFile, defineCustomElements } from "@vime/core";

  // 1. Manually define them to be as efficient as possible.
  customElements.define("vm-player", VmPlayer);
  customElements.define("vm-video", VmVideo);
  customElements.define("vm-file", VmFile);

  // 2. Can't be bothered? Load them all in, may bloat your final bundle size a little.
  defineCustomElements();
</script>

<main>
  <NavBar />
  <MediaPlayer />
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

  <!-- SMUI -->
  <link rel="stylesheet" href="https://unpkg.com/svelte-material-ui/bare.css" />
</svelte:head>
