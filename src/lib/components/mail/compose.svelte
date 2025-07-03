<script lang="ts">
  import Button from "../ui/button/button.svelte";
  import Pencil from "@lucide/svelte/icons/pencil";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import { useSidebar } from "$lib/components/ui/sidebar/index.js";

  const sidebar = useSidebar();
</script>

<div class="w-full p-1">
  <Tooltip.Root>
    <Tooltip.Trigger class="w-full">
      <Button
        class="w-full group-data-[collapsible=icon]:aspect-square group-data-[collapsible=icon]:p-2"
        variant="default"
        size="lg"
        onclick={() => {
          // Logic to open the compose email modal
          console.log("Compose email clicked");
        }}
      >
        <Pencil class="" />
        <span class="group-data-[collapsible=icon]:hidden">Compose</span>
      </Button>
    </Tooltip.Trigger>
    <Tooltip.Content
      side="right"
      hidden={sidebar.state !== "collapsed" || sidebar.isMobile}
    >
      Compose
    </Tooltip.Content>
  </Tooltip.Root>
</div>
