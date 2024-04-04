fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

use std::io;

fn main() {
    let input_string = "This is a test string with several words";
    match shortest_word(input_string) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the input string"),
    }
    println!("Enter the String");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read String");
    match shortest_word(&input) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the input string"),
    }
}
