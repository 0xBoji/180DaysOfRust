use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    
    if let Some(command) = args.get(1) {
        handle_command(command);
    } else {
        println!("No command provided. Usage: cargo run <command>");
    }
}

fn handle_command(command: &str) {
    let name = "Brad";
    let status = "100%";

    match command {
        "hello" => println!("Hi {}, how are you?", name),
        "status" => println!("Status is {}", status),
        _ => println!("'{}' is not a valid command", command),
    }
}

pub fn advanced_example() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <command> [args...]");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "greet" => handle_greet(&args),
        "calculate" => handle_calculate(&args),
        _ => println!("Unknown command: {}. Available commands: greet, calculate", command),
    }
}

fn handle_greet(args: &[String]) {
    match args.get(2) {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Usage: cargo run greet <name>"),
    }
}

fn handle_calculate(args: &[String]) {
    if args.len() < 4 {
        println!("Usage: cargo run calculate <num1> <num2>");
        return;
    }

    let num1: i32 = args[2].parse().unwrap_or_else(|_| {
        println!("Error: First number is not a valid integer");
        0
    });
    let num2: i32 = args[3].parse().unwrap_or_else(|_| {
        println!("Error: Second number is not a valid integer");
        0
    });

    println!("Sum: {}", num1 + num2);
}

/*
Key Points:
1. Command Line Arguments: We use env::args() to collect command-line arguments.
2. Error Handling: We provide informative error messages for invalid usage.
3. Pattern Matching: We use match statements for cleaner command handling.
4. Function Separation: We've separated concerns into different functions for better organization.
5. Argument Parsing: We use parse() to convert string arguments to integers, with error handling.
6. Slices: We use slices (&[String]) to pass arguments around, which is more flexible than Vec<String>.
7. Option and Result: We leverage Rust's Option and Result types for safer code.
8. Descriptive Comments: We've kept the key points to explain the code structure and Rust features used.
*/