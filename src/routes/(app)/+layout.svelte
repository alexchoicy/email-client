<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/navbar/app-sidebar.svelte";
  let { children } = $props();
  import { ModeWatcher } from "mode-watcher";
</script>

<ModeWatcher />
<Sidebar.Provider>
  <AppSidebar />
  <Sidebar.Inset>
    <Sidebar.Trigger class="-ml-1" />
    {@render children()}
  </Sidebar.Inset>
</Sidebar.Provider>
