use std::io;

/// Checks if a given string is a palindrome.
///
/// A palindrome is a word, phrase, number, or other sequence of characters
/// that reads the same forward and backward, ignoring spaces, punctuation,
/// and capitalization.
///
/// # Arguments
///
/// * `s` - A string slice that holds the sequence to check
///
/// # Returns
///
/// * `true` if the input is a palindrome, `false` otherwise
fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    
    cleaned.chars().eq(cleaned.chars().rev())
}

/// Runs the palindrome checker program.
///
/// This function prompts the user for input, reads the input from stdin,
/// checks if it's a palindrome, and prints the result.
pub fn run() {
    println!("Palindrome Checker");
    println!("=================");
    println!("Enter a string to check if it's a palindrome:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();
    
    let result = if is_palindrome(input) {
        "is"
    } else {
        "is not"
    };
    
    println!("'{}' {} a palindrome.", input, result);
}