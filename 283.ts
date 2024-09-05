/**
 Do not return anything, modify nums in-place instead.
 */
function moveZeroes(nums: number[]): number[] {
  let i = 0;
  let zeroes = 0;
  while (i < nums.length - zeroes) {
    if (nums[i] === 0) {
      nums.splice(i, 1);
      nums.push(0);
      zeroes++;
    } else {
      i++;
    }
  }
  return nums;
}

console.log(moveZeroes([0, 1, 0, 3, 12]));
