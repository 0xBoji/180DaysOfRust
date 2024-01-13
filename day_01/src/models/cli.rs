use std::env;

pub fn run() {
    // Collecting Command Line Arguments: Args are collected into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Accessing an Argument: The command is expected to be the second argument (index 1).
    if args.len() > 1 {
        let command = args[1].clone();
    } else {
        println!("No command provided");
        // Handle the case where no additional argument is provided
    }
    // Hardcoded Variables: 'name' and 'status' are example variables for demonstration.
    let command = args[0].clone();

    let name = "Brad";
    let status = "100%";

    // Branching Logic: Different output based on the command.
    if command == "hello" {
        // Greeting: If the command is "hello", it prints a greeting.
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        // Status: If the command is "status", it prints the status.
        println!("Status is {}", status);
    } else {
        // Invalid Command: If the command is neither, it indicates invalidity.
        println!("That is not a valid command");
    }
}

// Additional Example: Demonstrating more complex argument handling and branching.
pub fn advanced_example() {
    let args: Vec<String> = env::args().collect();

    // Checking for the Minimum Number of Arguments
    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "greet" => {
            // Command 'greet': Expects a name as the third argument.
            if args.len() < 3 {
                println!("Name not provided for 'greet'");
            } else {
                let name = &args[2];
                println!("Hello, {}!", name);
            }
        },
        "calculate" => {
            // Command 'calculate': Expects two numeric arguments for addition.
            if args.len() < 4 {
                println!("Not enough arguments for 'calculate'");
            } else {
                let num1: i32 = args[2].parse().unwrap_or(0);
                let num2: i32 = args[3].parse().unwrap_or(0);
                println!("Sum: {}", num1 + num2);
            }
        },
        _ => println!("Unknown command: {}", command),
    }
}

/* 
Key Points:
Command Line Arguments: The env::args() function collects command line arguments. The first argument is typically the path to the program, so real commands start from index 1.
Conditional Logic: The if-else statements are used to execute different blocks of code based on the command.
Match Statement: In the advanced example, a match statement is used for more complex command handling. It's more robust and cleaner, especially when dealing with multiple commands.
Argument Parsing: The parse() method converts string arguments into other types like integers. It's used in the 'calculate' command to perform arithmetic operations.
Error Handling: Basic error handling is demonstrated by checking the number of arguments and handling potential parsing errors with unwrap_or.
*/