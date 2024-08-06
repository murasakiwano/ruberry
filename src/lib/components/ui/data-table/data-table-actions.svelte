<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Ellipsis from "lucide-svelte/icons/ellipsis";
  import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuTrigger,
    DropdownMenuSeparator,
  } from "$lib/components/ui/dropdown-menu";

  export let id: string;
</script>

<DropdownMenu>
  <DropdownMenuTrigger asChild let:builder>
    <Button variant="ghost" builders={[builder]} size="icon" class="relative h-8 w-8 p-0">
      <span class="sr-only">Open menu</span>
      <Ellipsis class="h-4 w-4" />
    </Button>
  </DropdownMenuTrigger>
  <DropdownMenuContent>
    <DropdownMenuGroup>
      <DropdownMenuLabel>Actions</DropdownMenuLabel>
      <DropdownMenuItem on:click={() => navigator.clipboard.writeText(id)}>
        Copy transaction ID
      </DropdownMenuItem>
    </DropdownMenuGroup>
    <DropdownMenuSeparator />
    <DropdownMenuItem>View customer</DropdownMenuItem>
    <DropdownMenuItem>View transaction details</DropdownMenuItem>
  </DropdownMenuContent>
</DropdownMenu>
