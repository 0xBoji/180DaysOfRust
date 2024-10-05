// Variables in Rust: Fundamentals and Advanced Concepts
// This module demonstrates various aspects of variable usage in Rust

pub fn run() {
    basic_variable_usage();
    advanced_variable_usage();
}

// Demonstrates basic variable concepts in Rust
fn basic_variable_usage() {
    // Immutable variable: Cannot be changed after initialization
    let name: &str = "Peter";

    // Mutable variable: Can be modified
    let mut age: u32 = 24;
    println!("Initial: My name is {} and I am {} years old", name, age);

    // Modifying a mutable variable
    age = 23;
    println!("Updated: My name is {} and I am {} years old", name, age);

    // Constants: Immutable values known at compile time, must be type-annotated
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multiple assignment using tuple destructuring
    let (my_name, my_age): (&str, u32) = ("Peter", 24);
    println!("{} is {} years old", my_name, my_age);
}

// Demonstrates advanced variable concepts like shadowing and scoping
fn advanced_variable_usage() {
    // Shadowing: Reusing variable names
    let x = 5;
    let x = x + 1; // This shadows the previous 'x'

    {
        // Block scope: 'x' here is different from outer 'x'
        let x = x * 2;
        println!("Inner scope: x = {}", x);
    }

    println!("Outer scope: x = {}", x);

    // Loop scope demonstration
    for i in 0..3 {
        println!("Loop iteration: {}", i);
        // 'i' is only accessible within this loop
    }
    // 'i' is not accessible here
}

// Additional examples to showcase Rust's variable features
pub fn additional_examples() {
    // Type inference
    let inferred_type = 42; // Rust infers this as i32

    // Explicit type annotation
    let explicit_float: f64 = 3.14;

    // Shadowing with type change
    let spaces = "   ";
    let spaces = spaces.len(); // 'spaces' is now a number

    println!("Inferred: {}, Explicit: {}, Spaces: {}", inferred_type, explicit_float, spaces);

    // Using 'let' in conditional statements
    if let Some(x) = Some(5) {
        println!("Matched: {}", x);
    }

    // Pattern matching with destructuring
    let point = (3, 5);
    let (x, y) = point;
    println!("Point: ({}, {})", x, y);
}

/*
Key Concepts Demonstrated:
1. Immutability: Variables are immutable by default in Rust.
2. Mutability: Use 'mut' keyword for mutable variables.
3. Constants: Immutable values known at compile-time, always type-annotated.
4. Shadowing: Reusing variable names, potentially with different types.
5. Scoping: Variables are block-scoped in Rust.
6. Type Inference: Rust can infer types in most cases.
7. Explicit Type Annotation: When needed or for clarity.
8. Destructuring: Unpacking tuples or structs into distinct variables.
9. Pattern Matching: Used in various contexts, including 'if let' constructs.

This refactored code provides a more comprehensive overview of variable usage in Rust,
from basic concepts to more advanced features, with clear examples and explanations.
*/