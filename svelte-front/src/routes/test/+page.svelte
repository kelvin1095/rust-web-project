<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
  const help = data.helpme;
  console.log(help);
</script>

<svelte:head>
  <title>Test Page</title>
</svelte:head>

<style>
</style>
