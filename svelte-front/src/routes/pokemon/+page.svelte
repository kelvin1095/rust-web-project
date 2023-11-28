<script lang="ts">
  import { type1_store, type2_store, search_result_store } from "../store";
  import { onMount } from "svelte";

  type Pokemon = {
    pokedexnumber: number;
    name: string;
    form?: string;
    type1: string;
    type2?: string;
  };

  const pokemonTypes = [
    "",
    "Normal",
    "Fire",
    "Water",
    "Electric",
    "Grass",
    "Ice",
    "Fighting",
    "Poison",
    "Ground",
    "Flying",
    "Psychic",
    "Bug",
    "Rock",
    "Ghost",
    "Dragon",
    "Dark",
    "Steel",
    "Fairy",
  ];

  let type1 = "";
  let type2 = "";

  let result: Pokemon[] = [];

  onMount(() => {
    type1_store.subscribe((value) => {
      type1 = value;
    });

    type2_store.subscribe((value) => {
      type2 = value;
    });

    search_result_store.subscribe((value) => {
      result = value;
    });
  });

  const fetchData = async () => {
    const formData = {
      type1: type1,
      type2: type2,
    };

    try {
      const response = await fetch("/api/pokemon/type", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(formData),
      });

      // Handle the response as needed
      const data: Pokemon[] = await response.json();
      result = data;
    } catch (error) {
      console.error("Error:", error);
      result = [];
    }
  };

  const handleSubmit = async (event: Event) => {
    event.preventDefault();
    await fetchData();

    type1_store.set(type1);
    type2_store.set(type2);
    search_result_store.set(result);
  };
</script>

<svelte:head>
  <title>Pokemon</title>
</svelte:head>

<h1>Pokemon Search Page!</h1>
<p>
  On this page, I learnt how to use svelte stores to store search parameters and
  results (so that when you navigate away and back, you can continue like
  before) as well as connecting to a postgres db where all the pokemon data is
  stored.
</p>

<p>
  When you seach and click on any pokemon returned, it will display name, form,
  type, ability, weight, height and stats.
</p>

<p>
  The only possible issue to do here is when you search and click on a specific
  pokemon's form, it does not display automatically display that form.
</p>

<form on:submit={handleSubmit}>
  <label for="type1">Primary Type:</label>
  <select name="type1" id="type1" bind:value={type1}>
    {#each pokemonTypes as type, i}
      <option value={type}>{type}</option>
    {/each}
  </select>

  <label for="type2">Secondary Type:</label>
  <select name="type2" id="type2" bind:value={type2}>
    {#each pokemonTypes as type, i}
      <option value={type}>{type}</option>
    {/each}
  </select>

  <button>Submit!</button>
</form>

<div>
  <ul>
    {#each result as pokemon, i}
      <li>
        <a href="/pokemon/{pokemon.pokedexnumber}">
          #{pokemon.pokedexnumber.toString().padStart(4, "0")}
          {pokemon.name}
          {pokemon.form ? "(" + pokemon.form + ")" : ""}
        </a>
      </li>
    {/each}
  </ul>
</div>

<style>
  h1 {
    text-align: center;
  }

  form {
    text-align: center;
  }
</style>
