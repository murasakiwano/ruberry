<script lang="ts">
  import Money from "$lib/components/Money.svelte";
  import * as Table from "$lib/components/ui/table/index";
  import { type Transaction, invoke } from "$lib/invoke";

  let expenses: Transaction[] = [];

  async function getExpenses() {
    expenses = await invoke("get_expenses");
  }

  $: getExpenses().then(() => {});

  $: totalSum = expenses.map((tx) => tx.amount).reduce((prev, curr) => prev + curr, 0);
</script>

<div class="container">
  <h1 class="p-5 text-2xl">Lista de despesas</h1>
  <div class="">
    <Table.Root>
      <Table.Caption>Uma lista das suas despesas recentes</Table.Caption>
      <Table.Header>
        <Table.Row>
          <Table.Head class="w-[150px]">Data</Table.Head>
          <Table.Head>Transação</Table.Head>
          <Table.Head>Categoria</Table.Head>
          <Table.Head class="text-right">Quantia</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each expenses as expense, i (i)}
          <Table.Row>
            <Table.Cell class="font-medium">{expense.date}</Table.Cell>
            <Table.Cell>{expense.title}</Table.Cell>
            <Table.Cell>{expense.category}</Table.Cell>
            <Table.Cell class="text-right font-mono">
              <Money value={expense.amount} />
            </Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
      <Table.Footer>
        <Table.Row>
          <Table.Cell>Total</Table.Cell>
          <Table.Cell colspan={3} class="text-right font-mono">
            <Money value={totalSum} />
          </Table.Cell>
        </Table.Row>
      </Table.Footer>
    </Table.Root>
  </div>
</div>
