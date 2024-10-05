// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // If/Else: Multiple conditions checked using logical operators.
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If: Conditional expression that returns a boolean.
    let is_of_age = age >= 21;
    println!("Is Of Age: {}", is_of_age);

    // Demonstrate using a function for more complex logic
    print_drink_suggestion(age);
}

// Additional Example: Demonstrating more complex conditional logic.
pub fn complex_conditionals() {
    let number = 15;

    // Matching Multiple Conditions
    match number {
        0 => println!("Number is zero"),
        1..=9 => println!("Number is between 1 and 9"),
        10 | 11 => println!("Number is either 10 or 11"),
        12..=20 if number % 2 == 0 => println!("Number is even and between 12 and 20"),
        _ => println!("Number is something else"),
    }

    // Using 'match' as an Expression
    let description = match number {
        0 => "zero",
        1..=9 => "small",
        10 | 11 => "ten or eleven",
        _ => "large",
    };
    println!("The number is {}", description);

    // Demonstrate using a function with pattern matching
    describe_number(number);
}

// Helper function to suggest drinks based on age
fn print_drink_suggestion(age: u8) {
    match age {
        0..=12 => println!("How about a nice glass of milk?"),
        13..=20 => println!("May I suggest a refreshing soda?"),
        21..=64 => println!("Perhaps you'd like to try our craft beer selection?"),
        65..=u8::MAX => println!("Would you care for our finest whiskey, aged to perfection?"),
    }
}

// Helper function to describe a number using pattern matching
fn describe_number(n: i32) {
    let description = match n {
        n if n < 0 => "negative",
        0 => "zero",
        n if n % 2 == 0 => "positive and even",
        _ => "positive and odd",
    };
    println!("The number {} is {}", n, description);
}

/* 
Key Points:
1. Basic If/Else: Checks conditions using logical operators (&&, ||).
2. Shorthand If: A concise way to write an if statement that returns a value.
3. Match Statement: A powerful control flow construct for pattern matching.
4. Match with Ranges and Conditions: Can be used with value ranges and additional conditions.
5. Match as an Expression: Can be used to return a value.
6. Functions with Conditionals: Demonstrate how to use functions for more complex conditional logic.
7. Pattern Matching in Functions: Show how to use pattern matching within function definitions.
*/