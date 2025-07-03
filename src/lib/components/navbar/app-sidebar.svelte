<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { EmailProvider, type EmailAccount } from "$lib/types/account";
  import NavMain from "$lib/components/navbar/nav-main.svelte";
  import AccountSwitcher from "./account-switcher.svelte";
  import Compose from "$lib/components/mail/compose.svelte";
  import NavTags from "./nav-tags.svelte";

  import Settings from "@lucide/svelte/icons/settings";
  const emails: EmailAccount[] = [
    {
      id: "0",
      email: "",
      displayName: "All Accounts",
      provider: EmailProvider.All,
    },
    {
      id: "1",
      email: "user1asdasdasdasdasdasdasdasd@example.com",
      provider: EmailProvider.Gmail,
    },
    {
      id: "2",
      email: "user2@example.com",
      provider: EmailProvider.Outlook,
    },
  ];

  const tags = [
    {
      id: "0",
      name: "Ads",
    },
    {
      id: "1",
      name: "Social",
    },
    {
      id: "2",
      name: "Updates",
    },
  ];
</script>

<Sidebar.Root collapsible="icon">
  <Sidebar.Header>
    <AccountSwitcher accounts={emails} />
  </Sidebar.Header>
  <Sidebar.Separator />
  <Sidebar.Content>
    <Compose />
    <NavMain />
    <NavTags {tags} />
  </Sidebar.Content>
  <Sidebar.Footer class="mb-2">
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton tooltipContent="Settings">
          <Settings class="size-5" />
          <span>Settings</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
