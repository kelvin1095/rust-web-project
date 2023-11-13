import type { PageLoad } from "./$types";

function generateCardObjects(
  dimensions: number,
  maxValue: number
): {
  id: number;
  played: boolean;
  active: boolean;
  card_description: number[];
}[] {
  const cards: {
    id: number;
    played: boolean;
    active: boolean;
    card_description: number[];
  }[] = [];
  let id = 0;

  function generateVector(
    currentVector: number[],
    dimensionIndex: number
  ): void {
    if (dimensionIndex === dimensions) {
      cards.push({
        id: id++,
        played: false,
        active: false,
        card_description: currentVector.slice(),
      });
      return;
    }

    for (let value = 0; value <= maxValue; value++) {
      currentVector[dimensionIndex] = value;
      generateVector(currentVector, dimensionIndex + 1);
    }
  }

  generateVector(new Array(dimensions).fill(0), 0);

  return cards;
}

const dimensions = 4;
const maxValue = 2;

const generatedObjects = generateCardObjects(dimensions, maxValue);

export const load: PageLoad = () => {
  return {
    deck: {
      shuffled_cards: generatedObjects,
    },
  };
};
