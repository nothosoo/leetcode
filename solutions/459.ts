function repeatedSubstringPattern(s: string): boolean {
  for (let i = 1; i <= s.length / 2; i++) {
    if (s.length % i === 0) {
      let hasError = false;
      for (let j = 1; j < s.length / i; j++) {
        if (
          (0 === i - 1 ? s.charAt(0) : s.slice(0, i)) !==
          (0 === i - 1 ? s.charAt(j * i) : s.slice(j * i, j * i + i))
        ) {
          hasError = true;
          continue;
        }
      }
      if (!hasError) {
        return true;
      }
    }
  }
  return false;
}

console.log(repeatedSubstringPattern("abcabcabc")); // true
