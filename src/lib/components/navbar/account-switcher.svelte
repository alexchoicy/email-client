<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { getProviderIcon, getProviderName } from "$lib/email-providers";
  import { EmailProvider, type EmailAccount } from "$lib/types/account";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import Mails from "@lucide/svelte/icons/mails";
  import PlusIcon from "@lucide/svelte/icons/plus";

  let { accounts }: { accounts: EmailAccount[] } = $props();

  let activeAccount = $state(accounts[0]);
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <DropdownMenu.Root>
      <DropdownMenu.Trigger class="w-full h-full">
        {#snippet child({ props })}
          <Sidebar.MenuButton
            {...props}
            class="h-full w-full min-h-13 data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground group-data-[collapsible=icon]:justify-center group-data-[collapsible=icon]:px-2"
          >
            <div class="flex justify-center items-center size-8 shrink-0">
              {#if activeAccount.provider === EmailProvider.All}
                <Mails class="size-6" />
              {:else}
                <img
                  src={getProviderIcon(activeAccount.provider)}
                  alt={getProviderName(activeAccount.provider)}
                  class="size-5 dark:invert dark:brightness-0 dark:contrast-100"
                />
              {/if}
            </div>
            <div
              class="grid flex-1 text-left text-sm leading-tight group-data-[collapsible=icon]:hidden"
            >
              <span class="truncate font-medium">
                {activeAccount.displayName || activeAccount.email}
              </span>
              {#if activeAccount.provider !== EmailProvider.All}
                <span class="truncate text-xs text-muted-foreground">
                  {getProviderName(activeAccount.provider)}
                </span>
              {/if}
            </div>
            <ChevronsUpDownIcon
              class="ml-auto size-4 shrink-0 group-data-[collapsible=icon]:hidden"
            />
          </Sidebar.MenuButton>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content>
        <DropdownMenu.Label>Switch Accounts</DropdownMenu.Label>
        <DropdownMenu.Separator />

        {#each accounts as account, index (account.id)}
          <DropdownMenu.Item
            onSelect={() => (activeAccount = account)}
            class="gap-3 p-3"
          >
            <div class="size-6 flex justify-center items-center shrink-0">
              {#if account.provider === EmailProvider.All}
                <Mails class="size-6" />
              {:else}
                <img
                  src={getProviderIcon(account.provider)}
                  alt={getProviderName(account.provider)}
                  class="size-6 dark:invert dark:brightness-0 dark:contrast-100"
                />
              {/if}
            </div>
            <div class="grid flex-1 text-left text-sm leading-tight min-w-0">
              <span class="truncate font-medium">
                {account.displayName || account.email}
              </span>
              {#if account.provider !== EmailProvider.All}
                <span class="truncate text-xs text-muted-foreground">
                  {getProviderName(account.provider)}
                </span>
              {/if}
            </div>
          </DropdownMenu.Item>
        {/each}
        <DropdownMenu.Separator />
        <DropdownMenu.Item class="gap-3 p-3">
          <div
            class="flex size-6 items-center justify-center rounded-md border bg-background shrink-0"
          >
            <PlusIcon class="size-4" />
          </div>
          <div class="font-medium text-sm">Add account</div>
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </Sidebar.MenuItem>
</Sidebar.Menu>
