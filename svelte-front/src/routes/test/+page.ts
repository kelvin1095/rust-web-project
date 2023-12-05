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
  const data = await fetch(`/api/asdfa/example`);

  if (!data.ok) {
    throw error(data.status, "Connection Error");
  }

  const helpme: MixedType[] = await data.json();

  if (helpme.length == 0) {
    throw error(404, "Not Found");
  }

  return {
    helpme: helpme,
  };
};
