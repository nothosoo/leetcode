pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut remainder = 1;
    for i in (0..digits.len()).rev() {
        if digits[i] == 9 {
            remainder = 1;
            digits[i] = 0;
        } else {
            digits[i] += remainder;
            return digits;
        }
        if i == 0 && digits[i] == 0 && remainder == 1 {
            digits.insert(0, 1);
        }
    }
    digits
}

fn main() {
    println!("{:?}", plus_one(vec![1, 9, 9]));
}
