<script lang="ts">
  import Button from "../ui/button/button.svelte";
  import Pencil from "@lucide/svelte/icons/pencil";
</script>

<div class="w-full p-1">
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
</div>
