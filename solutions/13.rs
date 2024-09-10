// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
use std::collections::HashMap;
pub fn roman_to_int(s: String) -> i32 {
    let mut hash_map = HashMap::new();
    hash_map.insert('I', 1);
    hash_map.insert('V', 5);
    hash_map.insert('X', 10);
    hash_map.insert('L', 50);
    hash_map.insert('C', 100);
    hash_map.insert('D', 500);
    hash_map.insert('M', 1000);
    let mut occurence = 0;
    let mut total = 0;
    let mut prevChar = s.chars().nth(0).unwrap();
    for (i, c) in s.chars().enumerate() {
        if prevChar == c {
            occurence += 1;
        }
        if hash_map.get(&prevChar) < hash_map.get(&c) {
            total = total - hash_map.get(&prevChar).unwrap() * occurence;
            occurence = 1;
        }
        if hash_map.get(&prevChar) > hash_map.get(&c) {
            total = total + hash_map.get(&prevChar).unwrap() * occurence;
            occurence = 1;
        }
        prevChar = c;
        if i == s.len() - 1 {
            total = total + hash_map.get(&c).unwrap() * occurence;
        }
    }
    return total;
}

fn main() {
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}
