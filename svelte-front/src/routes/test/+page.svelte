<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    const help = data.sentence_data;
    let button = 0;
    $: to_display = help[button];
</script>

<svelte:head>
    <title>Test Page</title>
</svelte:head>

<h1>Test page displaying mixed vectors from rust</h1>

<p>{button}</p>

<div id="japanese">
    {#if to_display.SentenceType}
        {to_display.SentenceType.sentence}: {to_display.SentenceType
            .translation}
    {:else if to_display.WordType}
        {to_display.WordType.word}: {to_display.WordType.translation}
    {:else}
        Hello
    {/if}
</div>

<button on:click={() => (button += 1)}>click me</button>
<button on:click={() => (button = 0)}>reset</button>

<style>
    @font-face {
        font-family: "Noto Sans JP";
        font-style: normal;
        font-weight: 400;
        src: url("/font/NotoSansJP-Regular.ttf");
    }

    #japanese {
        font-family: "Noto Sans JP";
        font-size: xx-large;
    }
</style>
