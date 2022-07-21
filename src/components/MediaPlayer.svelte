<script lang="ts">
  import { ItemsStore } from "./store";
  console.log(ItemsStore[0]);
  const poster = "https://www.server.com/poster.jpg";
  let source: any;
  ItemsStore.subscribe((value) => {
    source = value;
  });
</script>

<vm-player autoplay muted>
  <vm-video {poster} cross-origin>
    <!-- Why `data-src`? Lazy loading. You can always use `src` if you don't need it. -->

    <source data-src={source} type="" />
    <!-- <Youtube videoId="DyTCOwB0DVw" /> -->
  </vm-video>

  <!-- Loads the default Vime UI. -->
  <vm-default-ui />
</vm-player>
