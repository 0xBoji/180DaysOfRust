pub fn run() {
    // Example 1: Basic shadowing
    let x = 5;
    println!("Original x: {}", x);

    let x = x * 2;
    println!("Shadowed x: {}", x);

    // Example 2: Shadowing with different types
    let spaces = "   ";
    println!("Original spaces (string): '{}'", spaces);

    let spaces = spaces.len();
    println!("Shadowed spaces (number): {}", spaces);

    // Example 3: Shadowing in a smaller scope
    let y = 10;
    println!("Outer y: {}", y);

    {
        let y = "inner";
        println!("Inner y: {}", y);
    }

    println!("Outer y still: {}", y);

    /*
    Shadowing in Rust allows you to declare a new variable with the same name as a previous variable.
    This is different from mutability:
    1. You use 'let' for each new declaration.
    2. You can change the type of the value but reuse the same name.
    3. The new variable shadows the previous one until the scope ends.

    Key benefits:
    - Reuse variable names without losing immutability.
    - Transform data types while keeping a meaningful variable name.
    - Create clearer, more readable code in certain scenarios.
    */
}
