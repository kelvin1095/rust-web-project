import { writable } from "svelte/store";

export type Pokemon = {
    pokedex_number: number;
    name: string;
    form?: string;
};

export const login_status_store = writable(
    localStorage.getItem("username") != null
);

export const type1_store = writable("");
export const type2_store = writable("");
export const search_result_store = writable<Pokemon[]>([]);
