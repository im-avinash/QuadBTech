fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    let mut i = 2;
    while i*i <= num{
        if num % i == 0 {
            return false;
        }
        i+=1;
    }
    return true;
}

use std::io;

fn main() {
    let num = 17;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }

    println!("Enter an Integer:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_err) => {
            println!("Error: Enter a valid integer!");
            return;
        }
    };
    if is_prime(number) {
        println!("{} is prime", number);
    } else {
        println!("{} is not prime", number);
    }
}
