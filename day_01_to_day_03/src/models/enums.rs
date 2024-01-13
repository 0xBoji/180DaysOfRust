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
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    // Moving Avatars: The 'move_avatar' function is called with different 'Movement' variants.
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

// Additional Example: Demonstrating an Enum with Data.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// Function to Handle Messages: Demonstrates how to handle enums with data.
fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to: {}, {}, {}", r, g, b),
    }
}

pub fn message_example() {
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 30 },
        Message::Write("Hello, world!".to_string()),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        handle_message(msg);
    }
}

/* 
Key Points:
Basic Enums: Enums are used to define a type by enumerating its possible variants (e.g., Movement).
Pattern Matching with Enums: The match statement is a powerful way to handle different enum variants. For each Movement variant, a different action is performed.
Enums with Data: Enums can also contain data. In the Message example, different variants hold different types of data: no data for Quit, named fields for Move, a String for Write, and three u8 values for ChangeColor.
Handling Enum Variants: The handle_message function shows how to perform different actions based on the data contained in each Message variant.
Iterating Over Enum Variants: In message_example, an array of Message variants is iterated over, demonstrating how each variant is handled differently.
*/