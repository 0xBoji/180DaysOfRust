// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Creating a String: 'hello' is a mutable String initialized with "Hello ".
    let mut hello = String::from("Hello ");

    // Getting Length: Prints the current length of 'hello'.
    println!("Length: {}", hello.len());

    // Pushing a Character: Appends a character 'W' to 'hello'.
    hello.push('W');

    // Pushing a String Slice: Appends a string slice "orld!" to 'hello'.
    hello.push_str("orld!");

    // Getting Capacity: Prints the current capacity in bytes of 'hello'.
    println!("Capacity: {}", hello.capacity());

    // Checking if Empty: Prints whether 'hello' is empty.
    println!("Is empty: {}", hello.is_empty());

    // Checking Contains: Checks if 'hello' contains the substring "World".
    println!("Contains 'World': {}", hello.contains("World"));

    // Replacing a Substring: Replaces "World" with "There" in 'hello'.
    println!("Replace: {}", hello.replace("World", "There"));

    // Iterating Over Words: Loops through the words in 'hello', split by whitespace.
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // String with Capacity: Creates a String 's' with a pre-allocated capacity of 10.
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion Testing: Verifies the length and capacity of 's'.
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    // Printing 's'.
    println!("{}", s);

    // Additional Example: Demonstrating more string methods.
    advanced_string_operations(&hello);
}

// Additional Function: Advanced String Operations
fn advanced_string_operations(input: &String) {
    // Trimming: Removes whitespace from the start and end of the string.
    println!("Trimmed: {}", input.trim());

    // Chars: Iterates over the characters of the string.
    println!("Chars:");
    for c in input.chars() {
        println!("{}", c);
    }

    // Converting to Upper Case: Creates a new string with all characters in uppercase.
    println!("Upper Case: {}", input.to_uppercase());
}

// Example Usage
pub fn additional_example() {
    run();
}
/*
Key Points:
String vs str: String is a growable, heap-allocated data structure, ideal for when you need to modify or own string data. In contrast, str (often seen as &str) is an immutable, fixed-length string reference.
Common Operations: The code demonstrates common operations like appending characters (push), appending strings (push_str), checking length (len), and capacity (capacity).
String Manipulation: It shows how to check for emptiness, search for substrings (contains), and replace parts of the string (replace).
Iteration: The code iterates over the words in a string with split_whitespace.
String with Capacity: Demonstrates creating a String with a specific capacity and performing assertion tests on its length and capacity.
Advanced Operations: Additional functions illustrate advanced operations like trimming, iterating over characters, and converting to uppercase.
*/