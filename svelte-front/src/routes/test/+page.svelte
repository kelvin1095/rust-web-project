<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    const QuizQuestion = data.sentence_data;

    let response: number[] = [];

    ///
    let test = ["hello", "my", "name", "is", "kelvin"];

    function getStringsAtIndex(array: string[], indices: number[]) {
        // Ensure that the indices are valid
        const validIndices = indices.filter(
            (index) => index >= 0 && index < array.length
        );

        // Map the valid indices to their corresponding strings
        const result = validIndices.map((index) => array[index]);

        // Join the strings to form the final output
        return result.join(" ");
    }

    // Example usage with indices 1, 3, and 2
    let selectedStrings = getStringsAtIndex(test, [1, 3, 2]);

    console.log(selectedStrings);
    ///

    function add_to_response(index: number) {
        if (response.includes(index)) {
            response.splice(response.indexOf(index), 1);
        } else {
            response.push(index);
        }
        response = response;
        console.log(response);
    }

    let button = 0;
    $: to_display = QuizQuestion[button];
    // let test = to_display.SentenceList.broken_down;

    function submit_answer() {
        button += 1;
        response = [];
    }
</script>

<svelte:head>
    <title>Test Page</title>
</svelte:head>

<h1>Test Quiz page</h1>

<p>Question {button + 1} out of {QuizQuestion.length}</p>

<div id="japanese">
    {#if to_display.SentenceList}
        <p id="text">{to_display.SentenceList.english_text}</p>
        <p>{to_display.SentenceList.translated_text}</p>
        <p>{response}</p>
        <div class="build-sentence">
            {#each to_display.SentenceList.broken_down as words, i}
                <button
                    class="sentence-broken-down"
                    on:click={() => {
                        add_to_response(i);
                    }}
                >
                    {words}
                </button>
            {/each}
        </div>
    {:else if to_display.WordList}
        <p id="word">
            {to_display.WordList.english}:
            {to_display.WordList.romanized}
        </p>
        <p id="translated-word">{to_display.WordList.translated}</p>
    {:else}
        Hello
    {/if}
</div>

<button on:click={submit_answer}>Next!</button>
<button on:click={() => (button = 0)}>Reset</button>

<p>
    Please don't let the sentence count go past 10. It errors and I'm too lazy
    to fix it right now.
</p>

<style>
    @font-face {
        font-family: "Noto Sans JP";
        font-style: normal;
        font-weight: 400;
        src: url("/font/NotoSansJP-Regular.ttf");
    }

    h1 {
        text-align: center;
    }

    #word {
        margin-top: 0;
    }

    #translated-word {
        margin-bottom: 0;
    }

    #japanese {
        font-family: "Noto Sans JP";
        font-size: xx-large;
        border-style: solid;
        padding: 3rem;
    }

    .build-sentence {
        display: flex;
        column-gap: 0.5rem;
    }

    #text {
        margin: 0;
    }

    .sentence-broken-down {
        font-family: "Noto Sans JP";
        display: flex;
        align-items: center;
        height: 3rem;
        font-size: large;
        background-color: white;
        border-radius: 0.5rem;
        padding: 1rem;
        box-shadow:
            5px 0 5px rgba(0, 0, 0, 0.5),
            5px 0 5px rgba(0, 0, 0, 0.5);
    }

    .sentence-broken-down:hover {
        background-color: aqua;
    }

    .sentence-broken-down:active {
        background-color: grey;
    }
</style>
