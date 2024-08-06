<script lang="ts">
  import LineChart from "lucide-svelte/icons/line-chart";

  import { DataTable } from "$lib/components/ui/data-table";
  import { invoke } from "$lib/invoke";

  const getExpenses = async () => {
    return invoke("get_expenses");
  };
</script>

<div class="flex min-h-screen w-full flex-col bg-muted/40">
  <aside class="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background sm:flex">
    <nav class="flex flex-col items-center gap-4 px-2 sm:py-5">
      <a
        href="##"
        class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
      >
        <LineChart class="h-4 w-4 transition-all group-hover:scale-110" />
        <span class="sr-only">Analisar Transações</span>
      </a>
    </nav>
  </aside>
  <div class="container mx-auto py-10">
    {#await getExpenses()}
      <p>Loading expenses...</p>
    {:then expenses}
      <DataTable data={expenses} />
    {/await}
  </div>
</div>
