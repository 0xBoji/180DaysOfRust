use std::fmt::Display;

pub fn examples() {
    println!("Lifetimes Examples:");
    
    // Example 1: Basic lifetime usage
    let string1 = String::from("short");
    let string2 = String::from("longer");
    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);

    // Example 2: Demonstrating lifetime constraints
    let string3 = String::from("very long string");
    let result2;
    {
        let _string4 = String::from("tiny");
        result2 = longest(&string3, &string3);
    }
    println!("Longest string from example 2: {}", result2);

    // Example 3: Using 'static lifetime
    let static_string: &'static str = "I have a static lifetime";
    let result3 = longest(static_string, "Short lived");
    println!("Longest string with static lifetime: {}", result3);

    // Example 4: Using longest_with_announcement
    let result4 = longest_with_announcement(&string1, &string2, "Comparing strings");
    println!("Longest string with announcement: {}", result4);
}

// The lifetime parameter 'a specifies that the references x and y
// must have the same lifetime, and the return value will have that same lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Additional function to demonstrate multiple lifetime parameters
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}