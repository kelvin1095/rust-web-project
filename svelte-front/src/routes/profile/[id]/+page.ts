import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({ fetch, params }) => {
    let username = localStorage.getItem("username");

    if (username != params.id) {
        throw error(401, "Unauthorised");
    }

    return {
        username: params.id,
    };
};
