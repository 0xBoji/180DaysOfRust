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
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)
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
}

/* 
Key Points:
Basic If/Else: Checks conditions using logical operators (&&, ||). In your example, it's used to determine what the bartender says based on the customer's age and whether they have checked their ID.
Shorthand If: A concise way to write an if statement that returns a value. In your case, it's used to determine if someone is of drinking age.
Match Statement: A powerful control flow construct in Rust that can be used for pattern matching. It's similar to a switch-case statement in other languages but more powerful.
Match with Ranges and Conditions: match can be used with value ranges (e.g., 1..=9) and additional conditions (e.g., number % 2 == 0).
Match as an Expression: In Rust, match can be used as an expression to return a value. This is shown in the second example where description is assigned the result of a match expression.
*/