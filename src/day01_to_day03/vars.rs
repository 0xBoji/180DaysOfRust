// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // Immutable Variable: 'name' is a string slice (&str) and cannot be altered.
    let name = "Peter";

    // Mutable Variable: 'age' is a mutable integer variable.
    let mut age = 24;
    println!("My name is {} and I am {}", name, age);

    // Modifying Mutable Variable: The value of 'age' is changed.
    age = 23;
    println!("My name is {} and I am {}", name, age);

    // Constant: 'ID' is a constant integer. Constants must be type-annotated.
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Destructuring Assignment: Assigns values to 'my_name' and 'my_age' simultaneously.
    let (my_name, my_age) = ("Peter", 24);
    println!("{} is {}", my_name, my_age);

    // Advanced Example: Showcasing shadowing and scope.
    advanced_variable_usage();
}

// Advanced Function: Demonstrates shadowing and scope in variable usage.
fn advanced_variable_usage() {
    let x = 5;

    // Shadowing: Re-declares 'x' with a new value. The original 'x' is shadowed by this new 'x'.
    let x = x + 1;

    {
        // Block Scope: This 'x' is only valid in this scope and shadows the outer 'x'.
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    // The outer 'x' is not affected by the inner scope.
    println!("The value of x is: {}", x);

    // Using a Loop: Demonstrates loop scope and variable declaration within a loop.
    for i in 0..5 {
        println!("Loop iteration: {}", i);
        // 'i' is only accessible within this loop.
    }
    // Note: 'i' is not accessible here, outside the loop.
}

/*

Your Rust code snippet demonstrates basic variable declarations, mutability, constants, and multiple variable assignments. Let's add contextual comments and an example to illustrate more advanced variable usage in Rust:

rust
Copy code
// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // Immutable Variable: 'name' is a string slice (&str) and cannot be altered.
    let name = "Peter";

    // Mutable Variable: 'age' is a mutable integer variable.
    let mut age = 24;
    println!("My name is {} and I am {}", name, age);

    // Modifying Mutable Variable: The value of 'age' is changed.
    age = 23;
    println!("My name is {} and I am {}", name, age);

    // Constant: 'ID' is a constant integer. Constants must be type-annotated.
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Destructuring Assignment: Assigns values to 'my_name' and 'my_age' simultaneously.
    let (my_name, my_age) = ("Peter", 24);
    println!("{} is {}", my_name, my_age);

    // Advanced Example: Showcasing shadowing and scope.
    advanced_variable_usage();
}

// Advanced Function: Demonstrates shadowing and scope in variable usage.
fn advanced_variable_usage() {
    let x = 5;

    // Shadowing: Re-declares 'x' with a new value. The original 'x' is shadowed by this new 'x'.
    let x = x + 1;

    {
        // Block Scope: This 'x' is only valid in this scope and shadows the outer 'x'.
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    // The outer 'x' is not affected by the inner scope.
    println!("The value of x is: {}", x);

    // Using a Loop: Demonstrates loop scope and variable declaration within a loop.
    for i in 0..5 {
        println!("Loop iteration: {}", i);
        // 'i' is only accessible within this loop.
    }
    // Note: 'i' is not accessible here, outside the loop.
}

// Example Usage
pub fn additional_example() {
    run();
}
Key Points:
Immutable Variables: By default, variables in Rust are immutable. This means once a value is bound to a variable name, it cannot be changed (e.g., name).
Mutable Variables: Using mut allows a variable to be mutable, meaning its value can be changed after it's declared (e.g., age).
Constants: Constants are always immutable and must be type-annotated. They have a fixed value throughout the program (e.g., ID).
Destructuring: Rust allows destructuring a tuple to simultaneously assign values to multiple variables.
Shadowing: A variable can be re-declared with the same name, shadowing the previous value. This is useful for converting types or making variables immutable after performing some computation.
Block Scope: Variables in Rust are scoped to the block they are declared in. Variables declared in a nested block are not accessible outside that block.
Loop Variables: Loop control variables are scoped to the loop and are not accessible outside it.
*/ 