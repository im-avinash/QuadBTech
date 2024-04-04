fn is_palindrome(str1: &str) -> bool {
    let str1 = str1.to_lowercase();
    let strAlphaNum: String = str1.chars().filter(|c| c.is_alphanumeric()).collect();
    let strRev: String = strAlphaNum.chars().rev().collect();
    strAlphaNum == strRev
}

use std::io;

fn main() {
    let string1 = "A man, a plan, a canal, Panama!";
    println!("{}", is_palindrome(string1)); // Expected Output: true

    let string2 = "Hello, world!";
    println!("{}", is_palindrome(string2)); // Expected Output: false

    println!("Please Enter String:");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to Read String");

    println!("{}", is_palindrome(&mut input));

}