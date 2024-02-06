import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

type PokemonInfo = {
    pokedex_number: number;
    name: string;
    form: string;
    category: string;
    type_1: string;
    type_2: string;
    ability_1: string;
    ability_2: string;
    hidden_ability: string;
    hp: number;
    att: number;
    def: number;
    spa: number;
    spd: number;
    spe: number;
    height: number;
    weight: number;
    pokemon_image: string;
};

export const load: PageLoad = async ({ fetch, params }) => {
    const pokemonData = await fetch(`/api/pokemon/${params.id}`);

    if (!pokemonData.ok) {
        error(pokemonData.status, "Connection Error");
    }

    const pokemonInfo: PokemonInfo[] = await pokemonData.json();

    if (pokemonInfo.length == 0) {
        error(pokemonData.status, "Not Found");
    }

    return {
        pokemonInfo: pokemonInfo,
    };
};
