import type { PageLoad } from "./$types";
import { error } from "@sveltejs/kit";

type WordList = {
    english: string;
    translated: string;
    romanized: string;
};

type SentenceList = {
    english_text: string;
    translated_text: string;
    broken_down: string[];
};

type MixedType = {
    WordList?: WordList[];
    SentenceList?: SentenceList;
};

type ApiResponseTemplate<T> = {
    status: string;
    message: string;
    data: T;
};

type ApiResponse = ApiResponseTemplate<MixedType[]>;

export const load: PageLoad = async ({ fetch }) => {
    let quiz_list: MixedType[] = await fetch(`/api/japanese/test`)
        .then((response) => {
            if (response.ok) {
                return response.json();
            }
            return response.json().then((errorData: ApiResponse) => {
                throw errorData;
            });
        })
        .then((data: ApiResponse) => {
            if (data.data.length == 0) {
                error(404, "Error");
            }
            return data.data;
        })
        .catch((error) => {
            throw error(404, "Not Found");
        })
        .finally(() => {});

    return {
        sentence_data: quiz_list,
    };
};
