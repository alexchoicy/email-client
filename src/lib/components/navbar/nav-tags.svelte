<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import ArchiveXIcon from "@lucide/svelte/icons/archive-x";
  import FileIcon from "@lucide/svelte/icons/file";
  import InboxIcon from "@lucide/svelte/icons/inbox";
  import SendIcon from "@lucide/svelte/icons/send";
  import Trash2Icon from "@lucide/svelte/icons/trash-2";

  const { tags } = $props();
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
  <Sidebar.GroupLabel>Navigation</Sidebar.GroupLabel>
  <Sidebar.GroupContent>
    <Sidebar.Menu>
      {#each tags as tag}
        <Sidebar.MenuItem>
          <Sidebar.MenuButton>
            <span>{tag.name}</span>
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      {/each}
    </Sidebar.Menu>
  </Sidebar.GroupContent>
</Sidebar.Group>
