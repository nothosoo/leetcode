pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort();
    arr.windows(2)
        .all(|window| window[0] - window[1] == arr[0] - arr[1])
}

fn main() {
    println!("{}", can_make_arithmetic_progression(vec![3, 5, 1]))
}
