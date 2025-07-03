<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import ArchiveXIcon from "@lucide/svelte/icons/archive-x";
  import FileIcon from "@lucide/svelte/icons/file";
  import InboxIcon from "@lucide/svelte/icons/inbox";
  import SendIcon from "@lucide/svelte/icons/send";
  import Trash2Icon from "@lucide/svelte/icons/trash-2";

  const options = [
    {
      title: "Inbox",
      url: "#",
      icon: InboxIcon,
      isActive: true,
    },
    {
      title: "Drafts",
      url: "#",
      icon: FileIcon,
      isActive: false,
    },
    {
      title: "Sent",
      url: "#",
      icon: SendIcon,
      isActive: false,
    },
    {
      title: "Junk",
      url: "#",
      icon: ArchiveXIcon,
      isActive: false,
    },
    {
      title: "Trash",
      url: "#",
      icon: Trash2Icon,
      isActive: false,
    },
  ];
</script>

<Sidebar.Group>
  <Sidebar.GroupLabel>Navigation</Sidebar.GroupLabel>
  <Sidebar.GroupContent>
    <Sidebar.Menu>
      {#each options as option}
        <Sidebar.MenuItem>
          <Sidebar.MenuButton tooltipContent={option.title}>
            <option.icon class="size-2" />
            <span>{option.title}</span>
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      {/each}
    </Sidebar.Menu>
  </Sidebar.GroupContent>
</Sidebar.Group>
