<script lang="ts">
  import { DataTable } from "$lib/components/ui/data-table";
  import { invoke } from "$lib/invoke";

  // import Money from "$lib/components/Money.svelte";

  // let expenses: Transaction[];
  const getExpenses = async () => {
    return invoke("get_expenses");
  };
</script>

<div class="container mx-auto py-10">
  {#await getExpenses()}
    <p>Loading...</p>
  {:then expenses}
    <DataTable data={expenses} />
  {/await}
</div>

<a href="/">Go Home</a>
