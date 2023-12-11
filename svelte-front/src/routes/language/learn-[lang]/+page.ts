import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ fetch, params }) => {
    let language = params.lang;

    return {
        selected: language,
    };
};
