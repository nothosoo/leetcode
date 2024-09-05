function mergeAlternately(word1: string, word2: string): string {
  let merged = word1 + word2;
  const loopLength = Math.min(word1.length, word2.length) * 2;
  for (let i = 1; i < loopLength + 1; i++) {
    if (i % 2 === 0) {
      merged =
        merged.substring(0, i - 1) +
        word2.charAt(i / 2 - 1) +
        merged.substring(i);
    } else {
      merged =
        merged.substring(0, i - 1) +
        word1.charAt(Math.floor(i / 2)) +
        merged.substring(i);
    }
  }
  if (word1.length > word2.length) {
    merged = merged.substring(0, loopLength) + word1.substring(word2.length);
  }
  return merged;
}

console.log(mergeAlternately("abcd", "pq"));
