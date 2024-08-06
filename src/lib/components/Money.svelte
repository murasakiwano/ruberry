<script lang="ts">
  export let value: number;
</script>

<span class="font-mono" {...$$restProps}>
  {value.toLocaleString("pt-br", {
    style: "currency",
    currency: "BRL",
  })}
</span>
