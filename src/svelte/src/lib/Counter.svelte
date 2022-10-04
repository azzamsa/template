<script lang="ts">
  let count = 0;
  const increment = () => {
    count += 1;
  };
</script>

<button on:click={increment} class="rounded bg-gray-200 p-2 outline hover:outline-orange-500">
  count is {count}
</button>
