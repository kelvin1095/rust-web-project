import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ params }) => {
    let valid_languages = ["japanese", "korean", "mandarin"];
    let language = params.lang;
    if (!valid_languages.includes(language)) {
        error(404, { message: "invalid language" });
    }

    return {
        selected: language.charAt(0).toUpperCase() + language.slice(1),
    };
};
