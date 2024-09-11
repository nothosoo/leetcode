pub fn length_of_last_word(s: String) -> i32 {
    // let words: Vec<&str> = s.split_whitespace().collect();
    // return words[words.len() - 1].len() as i32;
    s.split_whitespace()
        .rev()
        .next()
        .map(|word| word.len() as i32)
        .unwrap_or(0)
}

fn main() {
    println!(
        "{}",
        length_of_last_word(String::from("   fly me   to   the moon  "))
    )
}
