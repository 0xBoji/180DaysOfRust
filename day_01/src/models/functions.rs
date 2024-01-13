// Functions - Used to store blocks of code for re-use

pub fn run() {
    // Calling a Function: 'greeting' is called with two string literals as arguments.
    greeting("Hello", "Jane");

    // Function Return Value: The result of the 'add' function is stored in 'get_sum'.
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure: Closures are anonymous functions that can capture variables from their scope.
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

// Defining a Function: 'greeting' takes two string slices and prints a greeting message.
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// Another Function: 'add' takes two integers and returns their sum.
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

// Additional Example: A function that multiplies two numbers.
fn multiply(n1: i32, n2: i32) -> i32 {
    n1 * n2
}

// Example Usage
pub fn additional_example() {
    // Calling 'multiply' with two numbers.
    let result = multiply(6, 4);
    println!("Multiplication: {}", result);
}
/* 
Key Points:
Function Declaration: Functions are declared using fn keyword followed by the function name, parameters, and optionally a return type.
Calling Functions: Functions are called by their name and passing the required arguments, as seen with greeting and add.
Function Parameters: Parameters are specified in parentheses. In greeting, greet and name are parameters of type &str (string slices).
Return Values: The add function returns an integer. The return type is specified after the arrow ->. The last expression in a function is implicitly returned.
Closures: Closures are anonymous functions (function literals) that can capture variables from their surrounding scope. In this example, add_nums captures n3 from its enclosing function run.
Additional Example: The multiply function is an additional example demonstrating how functions can be defined and used to perform a specific task (in this case, multiplication).
*/