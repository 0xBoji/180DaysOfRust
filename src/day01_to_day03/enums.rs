// Enums are types which have a few definite values

// Defining an Enum: 'Movement' with four possible variants.
enum Movement {
    // Variants of the Movement Enum
    Up,
    Down,
    Left,
    Right,
}

// Function to Move an Avatar: Takes a 'Movement' enum as an argument.
fn move_avatar(m: Movement) {
    // Matching the Movement Enum: Executes different code based on the variant of 'm'.
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    // Creating Enum Instances: Each 'avatar' variable is assigned a different variant of the 'Movement' enum.
    let movements = vec![Movement::Left, Movement::Up, Movement::Right, Movement::Down];

    // Moving Avatars: The 'move_avatar' function is called with different 'Movement' variants.
    for movement in movements {
        move_avatar(movement);
    }
}

// Additional Example: Demonstrating an Enum with Data.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Function to Handle Messages: Demonstrates how to handle enums with data.
fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting the application"),
        Message::Move { x, y } => println!("Moving to coordinates: ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB: ({}, {}, {})", r, g, b),
    }
}

pub fn message_example() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        handle_message(msg);
    }
}

/* 
Key Points:
1. Enum Basics: Enums in Rust define a type by listing its possible variants. The 'Movement' enum demonstrates this with directional variants.

2. Pattern Matching: The 'match' expression is crucial for handling enum variants. It ensures all cases are covered, providing type-safe code.

3. Enums with Associated Data: The 'Message' enum shows how variants can hold different types of data, from simple structs to tuples and unit variants.

4. Flexible Data Structures: Enums allow for the creation of flexible data structures that can represent multiple states or types within a single type.

5. Iterating Over Enums: The refactored 'run' and 'message_example' functions demonstrate how to work with collections of enum variants, showcasing Rust's powerful iteration capabilities.

6. Rust's Type System: Enums are a key part of Rust's strong type system, allowing for expressive and safe code by encoding semantic meaning into the type itself.

7. Code Organization: Enums help in organizing related concepts under a single type, improving code readability and maintainability.

Context:
Enums in Rust are particularly powerful compared to enums in many other languages. They can contain data and methods, making them similar to algebraic data types in functional programming languages. This makes them ideal for representing state machines, command patterns, or any scenario where you have a fixed set of variants that might contain data.

The refactored code demonstrates more idiomatic Rust practices, such as using vectors and iterators instead of individual variables, which can make the code more scalable and easier to maintain.
*/