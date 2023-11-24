<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
  const language = data.selected;
</script>

<h1>Hello! This is where you will learn {language}</h1>

<style>
  h1 {
    text-align: center;
  }
</style>
