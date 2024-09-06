function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) return false;
  const letterHash = {};
  for (let i = 0; i < s.length; i++) {
    if (letterHash[s.charAt(i)] === undefined) {
      letterHash[s.charAt(i)] = 0;
    }
    if (letterHash[t.charAt(i)] === undefined) {
      letterHash[t.charAt(i)] = 0;
    }
    letterHash[s.charAt(i)]++;
    letterHash[t.charAt(i)]--;
  }
  return Object.keys(letterHash).every((key) => letterHash[key] === 0);
}

console.log(isAnagram("anagtam", "nbgbram")); // true
