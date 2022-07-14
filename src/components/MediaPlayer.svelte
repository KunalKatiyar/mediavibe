<script lang="ts">
  import {ItemsStore} from "./store";
  console.log(ItemsStore[0])
  const poster = "https://www.server.com/poster.jpg";
  let source = $ItemsStore[0].selected;
</script>

<vm-player autoplay muted>
  <vm-video poster={poster} cross-origin>
    <!-- Why `data-src`? Lazy loading. You can always use `src` if you don't need it. -->
    <source data-src={source} type="video/mp4" />
  </vm-video>

  <!-- Loads the default Vime UI. -->
  <vm-default-ui />
</vm-player>
