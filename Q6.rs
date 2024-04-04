fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first_str = &strings[0];
    for (i, &byte) in first_str.as_bytes().iter().enumerate() {
        for string in &strings[1..] {
            if i >= string.len() || byte != string.as_bytes()[i] {
                return first_str[..i].to_string();
            }
        }
    }
    first_str.to_string()
}

fn main() {
    let words = vec!["apple".to_string(), "app".to_string(), "ape".to_string()];
    let prefix = longest_common_prefix(&words);
    println!("Longest common prefix: {}", prefix);
}
