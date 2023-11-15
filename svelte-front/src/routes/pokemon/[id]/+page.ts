import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ fetch, params }) => {
  const pokemonData = await fetch(`/api/pokemon/${params.id}`);
  console.log(pokemonData);

  if (!pokemonData.ok) {
    throw error(pokemonData.status, "Connection Error");
  }

  const pokemonInfo = await pokemonData.json();

  if (pokemonInfo.length == 0) {
    throw error(pokemonData.status, "Not Found");
  }

  return {
    pokemonInfo: pokemonInfo,
  };
};
