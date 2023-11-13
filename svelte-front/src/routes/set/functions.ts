export function shuffleArray(array: any[]): any[] {
  let currentIndex = array.length;
  let randomIndex, temporaryValue;

  while (currentIndex !== 0) {
    randomIndex = Math.floor(Math.random() * currentIndex);
    currentIndex--;

    temporaryValue = array[currentIndex];
    array[currentIndex] = array[randomIndex];
    array[randomIndex] = temporaryValue;
  }

  return array;
}

export function addVectors(vector1: number[], vector2: number[]) {
  if (vector1.length !== vector2.length) {
    throw new Error(
      "Vectors must have the same dimensions for component-wise addition."
    );
  }

  const result = [];

  for (let i = 0; i < vector1.length; i++) {
    result.push((vector1[i] + vector2[i]) % 3);
  }

  return result;
}
