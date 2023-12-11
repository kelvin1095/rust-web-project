<script lang="ts">
    import type { PageData } from "./$types";
    import Card from "./card.svelte";
    import { shuffleArray, addVectors } from "./functions";
    export let data: PageData;

    // TO DO
    // Hint button
    // End of game conditions

    // Want to do
    // Do a better job of the svg images
    // My code is just a mess

    let deck = data.deck.shuffled_cards;
    let start_game = true;
    let last_information = "Press New Game to Begin";
    let additional_open = 0;
    let score = 0;

    function new_game() {
        start_game = false;
        deck = shuffleArray(deck);
        deck.map((card) => {
            card.played = false;
            card.active = false;
        });
        additional_open = 0;
        score = 0;
        last_information = "Select 3 Cards";
        return deck;
    }

    function increment(id: number) {
        deck.filter((card) => card.id === id).map(
            (card) => (card.active = !card.active)
        );
        deck = deck;

        let cards_selected = deck.filter((card) => card.active === true);

        if (cards_selected.length >= 3) {
            let match_condition = cards_selected
                .map((card) => card.card_description)
                .reduce(
                    (acc, card_description) =>
                        addVectors(acc, card_description),
                    [0, 0, 0, 0]
                )
                .reduce(
                    (accumulator: number, currentValue: number) =>
                        accumulator + currentValue,
                    0
                );

            if (match_condition === 0) {
                last_information = "Set!";
                deck.filter((card) => card.active === true).map((card) => {
                    card.played = true;
                    card.active = false;
                });

                additional_open = Math.max(0, additional_open - 1);
                score += 1;
            } else {
                last_information = "No Set";
                deck.filter((card) => card.active === true).map(
                    (card) => (card.active = false)
                );
            }
        }
    }

    $: cards_remaining = deck.filter((card) => card.played === false).length;
</script>

<svelte:head>
    <title>Set Card Game</title>
</svelte:head>

<h1>Play Set</h1>

<p>
    On this page, I've written up a game of set. Currently the "Hint" button
    does nothing and I have no code to handle end of game. There should also be
    some limit on the number of cards you can open.
</p>

<div id="game-buttons">
    <button on:click={() => new_game()}>New Game</button>
    <button>Hint</button>
    <button
        on:click={() => {
            additional_open += 1;
            score = Math.max(0, score - 1);
        }}>Open More Cards</button
    >
</div>

<div id="set-information">
    <h1>{last_information}</h1>
</div>

{#if start_game}
    <div />
{:else}
    <div id="game-information">
        <div id="score">
            <h1>Score: {score}</h1>
        </div>
        <div id="cards-remaining">
            <h1>Cards Remaining: {cards_remaining}</h1>
        </div>
    </div>

    <div id="game-grid">
        {#each deck
            .filter((card) => card.played == false)
            .slice(0, 12 + 3 * additional_open) as card (card.id)}
            <button
                class="card-in-play {card.active}"
                on:click={() => increment(card.id)}
            >
                <Card data={card.card_description} />
            </button>
        {/each}
    </div>
{/if}

<style>
    h1 {
        text-align: center;
    }

    #game-grid {
        display: grid;
        border-style: solid;
        border-width: 0.125rem;
        padding: 0.25rem;
        grid-template-columns: repeat(3, 1fr);
        gap: 0.25rem;
    }

    .card-in-play {
        background-color: white;
        border-style: dashed;
        border-radius: 1rem;
        border-width: 0.125rem;
        display: flex;
        justify-content: center;
        align-items: center;
        height: 9rem;
    }

    .true {
        background-color: azure;
    }

    .helpers {
        height: 0;
        width: 0;
    }

    #game-buttons {
        display: flex;
        column-gap: 0.5rem;
        padding: 1.5rem 0;
    }

    #game-buttons > button {
        flex: 1 1 0px;
        height: 3rem;
        background-color: white;
        border-radius: 1rem;
    }

    #set-information {
        padding: 1rem 0;
    }

    #game-information {
        padding: 1rem 0;
        display: flex;
        column-gap: 0.25rem;
    }

    #game-information > div {
        flex: 1 1 0px;
    }
</style>
