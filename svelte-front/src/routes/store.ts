import { writable } from "svelte/store";

type Pokemon = {
  pokedexnumber: number;
  name: string;
  form?: string;
  type1: string;
  type2?: string;
};

export const type1_store = writable("");
export const type2_store = writable("");
export const search_result_store = writable<Pokemon[]>([]);
