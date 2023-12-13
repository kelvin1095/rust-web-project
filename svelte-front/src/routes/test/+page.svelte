<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    const QuizQuestion = data.sentence_data;
    let button = 0;
    $: to_display = QuizQuestion[button];
</script>

<svelte:head>
    <title>Test Page</title>
</svelte:head>

<h1>Test page displaying mixed vectors from rust</h1>

<p>{button}</p>

<div id="japanese">
    {#if to_display.SentenceList}
        <p>{to_display.SentenceList.english_text}:</p>
        <p>{to_display.SentenceList.translated_text}</p>
        <ul>
            {#each to_display.SentenceList.broken_down as words}
                <li>{words}</li>
            {/each}
        </ul>
    {:else if to_display.WordList}
        <p>
            {to_display.WordList.english}:
            {to_display.WordList.translated}
        </p>
        <p>{to_display.WordList.romanized}</p>
    {:else}
        Hello
    {/if}
</div>

<button on:click={() => (button += 1)}>Next!</button>
<button on:click={() => (button = 0)}>reset</button>

<p>
    Please dont let the sentence count go past 10, it errors and I'm too lazy to
    fix it right now.
</p>

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
