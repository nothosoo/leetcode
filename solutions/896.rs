pub fn is_monotonic(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    let mut firstOccured = false;
    let mut isIncreasing = true;
    for i in 0..nums.len() - 1 {
        if !firstOccured {
            if nums[i] < nums[i + 1] {
                firstOccured = true;
                isIncreasing = true;
            } else if nums[i] > nums[i + 1] {
                firstOccured = true;
                isIncreasing = false;
            }
        } else {
            if nums[i] == nums[i + 1] {
                continue;
            }
            if (nums[i] < nums[i + 1] && !isIncreasing) || (nums[i] > nums[i + 1] && isIncreasing) {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    println!("{}", is_monotonic(vec![1, 2, 2, 3]));
}
