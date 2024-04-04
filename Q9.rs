fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = String::from("Hello, World!");
    let reversed = reverse_string(s);
    println!("Reversed string: {}", reversed);
}
