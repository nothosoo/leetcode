function strStr(haystack: string, needle: string): number {
  let startCursor = 0;
  let failIndex: number | undefined = undefined;
  let i = 0;
  while (i < haystack.length) {
    if (haystack.charAt(i) === needle.charAt(startCursor)) {
      startCursor++;
      i++;
    } else {
      startCursor = 0;
      i = failIndex === undefined ? i + 1 : failIndex;
      failIndex = undefined;
    }
    if (startCursor === needle.length) {
      return i - needle.length;
    }
    if (
      startCursor > 0 &&
      failIndex === undefined &&
      needle.charAt(0) === haystack.charAt(i)
    ) {
      failIndex = i;
    }
  }
  return -1;
}

console.log(strStr("mississippi", "sipp"));
