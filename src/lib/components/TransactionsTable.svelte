<script lang="ts">
  import Money from "$lib/components/Money.svelte";
  import * as Table from "$lib/components/ui/table/index";
  import { type Transaction, invoke } from "$lib/invoke";

  let transactions: Transaction[] = [];

  async function getTransactions() {
    transactions = await invoke("get_transactions");
  }

  getTransactions().then(() => {});

  $: totalSum = transactions.map((tx) => tx.amount).reduce((prev, curr) => prev + curr, 0);
</script>

<div class="container">
  <h1>Welcome to Ruberry!</h1>
  <Table.Root>
    <Table.Caption>Uma lista das suas transações recentes</Table.Caption>
    <Table.Header>
      <Table.Row>
        <Table.Head class="w-[150px]">Data</Table.Head>
        <Table.Head>Transação</Table.Head>
        <Table.Head>Categoria</Table.Head>
        <Table.Head class="text-right">Quantia</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#each transactions as transaction, i (i)}
        <Table.Row>
          <Table.Cell class="font-medium">{transaction.date}</Table.Cell>
          <Table.Cell>{transaction.title}</Table.Cell>
          <Table.Cell>{transaction.category}</Table.Cell>
          <Table.Cell class="text-right font-mono">
            <Money value={transaction.amount} />
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
