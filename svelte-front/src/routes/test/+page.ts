import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

type WordType = {
    word: string;
    translation: string;
};

type SentenceType = {
    sentence: string;
    translation: string;
};

type MixedType = {
    WordType?: WordType;
    SentenceType?: SentenceType;
};

export const load: PageLoad = async ({ fetch, params }) => {
    let help_me: MixedType[] = await fetch(`/api/example/example`)
        .then((response) => {
            if (!response.ok) {
                throw error(response.status, "Connection Error");
            }
            return response.json();
        })
        .then((data) => {
            if (data.length == 0) {
                throw error(404, "Not Found");
            }
            return data;
        })
        .catch((error) => {
            throw error(404, "Not Found");
        })
        .finally(() => {});

    console.log(help_me);

    return {
        sentence_data: help_me,
    };
};
