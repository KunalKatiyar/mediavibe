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
    <source data-src={source} type="" />
  </vm-video>
  <vm-default-ui />
</vm-player>
