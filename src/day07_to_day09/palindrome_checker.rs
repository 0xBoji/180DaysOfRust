use std::io;

fn is_palindrome(s: &str) -> bool {
    let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let s = s.to_lowercase();
    s.chars().eq(s.chars().rev())
}

pub fn run() {
    println!("Enter a string to check if it's a palindrome:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    let input = input.trim();
    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}