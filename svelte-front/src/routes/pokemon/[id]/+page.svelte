<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
    const pokedexData = data.pokemonInfo;

    let formSelect = 0;
    $: pokemon = pokedexData[formSelect];
</script>

<svelte:head>
    <title>{pokemon.name}</title>
</svelte:head>

<div>
    <h1>
        #{pokemon.pokedex_number.toString().padStart(4, "0")}
        {pokemon.name}
        {pokemon.form ? "(" + pokemon.form + ")" : ""}
    </h1>
    <h2>
        {pokemon.category}
    </h2>
    <div class="image">
        <img
            src="/image/PokemonHome/{pokemon.pokemon_image}"
            alt={pokemon.name}
            height="512"
        />
    </div>

    <div class="form-button-group">
        {#each { length: pokedexData.length } as _, i}
            <button on:click={() => (formSelect = i)} class="form-button">
                {i + 1}
            </button>
        {/each}
    </div>

    <table>
        <thead>
            <tr>
                <th>Type</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>
                    <div>
                        {pokemon.type_1}
                        {pokemon.type_2 || ""}
                    </div>
                </td>
            </tr>
        </tbody>
    </table>

    <table>
        <thead>
            <tr>
                <th>Height</th>
                <th>Weight</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>
                    {pokemon.weight ? pokemon.height : pokemon.height + "+"} m
                </td>
                <td>
                    {pokemon.weight || "???"} kg
                </td>
            </tr>
        </tbody>
    </table>

    <table>
        <thead>
            <tr>
                <th>Ability 1</th>
                <th>Ability 2</th>
                <th>Hidden Ability</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>{pokemon.ability_1}</td>
                <td>{pokemon.ability_2 || ""}</td>
                <td>{pokemon.hidden_ability || ""}</td>
            </tr>
        </tbody>
    </table>

    <table>
        <thead>
            <tr>
                <th>HP</th>
                <th>Attack</th>
                <th>Defense</th>
                <th>Sp. Atk</th>
                <th>Sp. Def</th>
                <th>Speed</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>{pokemon.hp}</td>
                <td>{pokemon.att}</td>
                <td>{pokemon.def}</td>
                <td>{pokemon.spa}</td>
                <td>{pokemon.spd}</td>
                <td>{pokemon.spe}</td>
            </tr>
        </tbody>
    </table>
</div>

<style>
    h1 {
        text-align: center;
    }

    h2 {
        text-align: center;
    }

    .image {
        text-align: center;
    }

    table {
        width: 100%;
        table-layout: fixed;
        border-style: solid;
    }

    td {
        text-align: center;
    }

    .form-button-group {
        text-align: center;
    }

    .form-button {
        width: 3em;
    }
</style>
