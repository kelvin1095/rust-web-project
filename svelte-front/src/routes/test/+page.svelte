<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    const QuizQuestion = data.sentence_data;

    console.log(QuizQuestion);

    let response: number[] = [];

    function getStringsAtIndex(array: string[], indices: number[]) {
        const validIndices = indices.filter(
            (index) => index >= 0 && index < array.length
        );
        const result = validIndices.map((index) => array[index]);

        return result.join(" ");
    }

    function add_to_response(index: number) {
        if (response.includes(index)) {
            response.splice(response.indexOf(index), 1);
        } else {
            response.push(index);
        }
        response = response;
    }

    let button = 0;
    $: to_display = QuizQuestion[button];

    function submit_answer() {
        console.log(response);
        button += 1;
        response = [];

        // fetch("/api/sum", {
        //     method: "POST",
        //     headers: {
        //         "Content-Type": "application/json",
        //     },
        //     body: JSON.stringify(formData),
        // })
        //     .then((response) => {
        //         if (response.ok) {
        //             return response.json();
        //         }
        //         return response.json().then((errorData: ApiResponse) => {
        //             throw errorData;
        //         });
        //     })
        //     .then((body: ApiResponse) => {
        //         result = `The result is: ${body.data.total}`;
        //     })
        //     .catch((error: ApiResponse) => {
        //         result = `Error: ${error.message}`;
        //     })
        //     .finally(() => {
        //         console.log("finally");
        //     });
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
        <div id="response">
            <p id="response-sentence">
                {getStringsAtIndex(
                    to_display.SentenceList.broken_down,
                    response
                )}
            </p>
        </div>
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
        <ul id="word">
            {#each to_display.WordList as { english }}
                <li>
                    <button>
                        {english}
                    </button>
                </li>
            {/each}
        </ul>

        <ul id="translated">
            {#each to_display.WordList as { translated }}
                <li>
                    <button>
                        {translated}
                    </button>
                </li>
            {/each}
        </ul>
    {:else}
        Hello
    {/if}
</div>

<button on:click={submit_answer}>Next!</button>
<button on:click={() => (button = 0)}>Reset Quiz</button>

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
        font-family: "Noto Sans JP";
    }

    #translated {
        font-family: "Noto Sans JP";
    }

    #response {
        display: flex;
        border-style: solid;
        height: 8rem;
        align-items: center;
    }

    #response-sentence {
        padding: 12rem;
    }

    #japanese {
        font-family: "Noto Sans JP";
        font-size: xx-large;
        border-style: solid;
        border-radius: 40px;
        padding: 3rem;
        margin: 1rem;
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
        margin-top: 3rem;
    }

    .sentence-broken-down:hover {
        background-color: aqua;
    }

    .sentence-broken-down:active {
        background-color: grey;
    }
</style>
