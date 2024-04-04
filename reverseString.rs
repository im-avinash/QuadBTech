fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}

use std::io;

fn main() {
    println!("Enter the String");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read String");
    let reversed = reverse_string(&input);
    println!("Original string: {}", &input);
    println!("Reversed string: {}", reversed);
    
}