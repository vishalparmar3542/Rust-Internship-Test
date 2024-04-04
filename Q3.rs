fn shortest_word(s: &str) -> &str {
    s.split_whitespace().min_by_key(|&word| word.len()).unwrap_or("")
}

fn main() {
    let test_string = "The quick brown fox jumps over the lazy dog";
    let shortest = shortest_word(test_string);
    println!("Shortest word: {}", shortest);
}
