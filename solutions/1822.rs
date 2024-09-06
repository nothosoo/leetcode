pub fn array_sign(nums: Vec<i32>) -> i32 {
    let mut return_value = 1;
    for num in nums.iter() {
        if *num == 0 {
            return 0;
        } else if *num < 0 {
            return_value *= -1;
        } else {
            return_value *= 1;
        }
    }
    return_value
}

fn main() {
    println!("{}", array_sign(vec![-1, -2, -3, -4, 3, 2, 1]));
}
