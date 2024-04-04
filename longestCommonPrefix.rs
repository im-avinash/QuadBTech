fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut prefix = String::new();

    for (i, c) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(ch) = string.chars().nth(i) {
                if ch != c {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings)); // Output: "fl"
}
