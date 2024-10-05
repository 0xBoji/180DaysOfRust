// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Demonstrating basic String operations
    basic_string_operations();

    // Demonstrating advanced String operations
    advanced_string_operations();
}

fn basic_string_operations() {
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
}

fn advanced_string_operations() {
    let hello = String::from("Hello World!");

    // Trimming: Removes whitespace from the start and end of the string.
    println!("Trimmed: {}", hello.trim());

    // Chars: Iterates over the characters of the string.
    println!("Chars:");
    for c in hello.chars() {
        println!("{}", c);
    }

    // Converting to Upper Case: Creates a new string with all characters in uppercase.
    println!("Upper Case: {}", hello.to_uppercase());

    // Additional advanced operations
    println!("Lower Case: {}", hello.to_lowercase());
    println!("Starts with 'Hello': {}", hello.starts_with("Hello"));
    println!("Ends with 'World!': {}", hello.ends_with("World!"));

    // Splitting string
    let split: Vec<&str> = hello.split(' ').collect();
    println!("Split words: {:?}", split);

    // Finding substrings
    if let Some(index) = hello.find("World") {
        println!("'World' starts at index: {}", index);
    }

    // Slicing
    println!("First five characters: {}", &hello[0..5]);
}

/*
Key Points:
1. String vs str: 
   - String is a growable, heap-allocated data structure, ideal for when you need to modify or own string data. 
   - str (often seen as &str) is an immutable, fixed-length string reference.

2. Common Operations: 
   - Appending characters (push)
   - Appending strings (push_str)
   - Checking length (len)
   - Checking capacity (capacity)

3. String Manipulation: 
   - Checking for emptiness (is_empty)
   - Searching for substrings (contains)
   - Replacing parts of the string (replace)

4. Iteration: 
   - Over words (split_whitespace)
   - Over characters (chars)

5. String Creation:
   - From a literal (String::from)
   - With capacity (String::with_capacity)

6. Advanced Operations:
   - Trimming whitespace (trim)
   - Case conversion (to_uppercase, to_lowercase)
   - Prefix/Suffix checking (starts_with, ends_with)
   - Splitting (split)
   - Finding substrings (find)
   - Slicing (&str[start..end])

7. Memory Management:
   - Strings in Rust are UTF-8 encoded
   - Capacity vs Length: Capacity is the amount of memory allocated, length is the current size of the string
   - Rust ensures memory safety with its ownership and borrowing rules

8. Performance Considerations:
   - Use String when you need to own or modify the string
   - Use &str when you only need to read the string
   - Pre-allocating capacity can improve performance when you know the final size of the string
*/