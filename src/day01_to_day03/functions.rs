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

    // Demonstrating the use of the multiply function
    let product = multiply(6, 4);
    println!("Product: {}", product);

    // Using the is_even function
    let number = 7;
    println!("Is {} even? {}", number, is_even(number));
}

/// Prints a greeting message.
///
/// # Arguments
///
/// * `greet` - A string slice that holds the greeting word
/// * `name` - A string slice that holds the name of the person to greet
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

/// Adds two integers and returns the sum.
///
/// # Arguments
///
/// * `n1` - The first integer
/// * `n2` - The second integer
///
/// # Returns
///
/// The sum of `n1` and `n2`
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

/// Multiplies two integers and returns the product.
///
/// # Arguments
///
/// * `n1` - The first integer
/// * `n2` - The second integer
///
/// # Returns
///
/// The product of `n1` and `n2`
fn multiply(n1: i32, n2: i32) -> i32 {
    n1 * n2
}

/// Checks if a number is even.
///
/// # Arguments
///
/// * `n` - The integer to check
///
/// # Returns
///
/// `true` if the number is even, `false` otherwise
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Example Usage
pub fn additional_example() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Using a closure with map to double each number
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    // Using filter with is_even function
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| is_even(x)).collect();
    println!("Even numbers: {:?}", even_numbers);
}

/*
Key Points:
1. Function Declaration: Functions are declared using the `fn` keyword, followed by the function name, parameters, and optionally a return type.
2. Calling Functions: Functions are called by their name and passing the required arguments, as seen with `greeting` and `add`.
3. Function Parameters: Parameters are specified in parentheses. In `greeting`, `greet` and `name` are parameters of type `&str` (string slices).
4. Return Values: Functions like `add` and `multiply` return an integer. The return type is specified after the arrow `->`. The last expression in a function is implicitly returned.
5. Closures: Closures are anonymous functions that can capture variables from their surrounding scope. In this example, `add_nums` captures `n3` from its enclosing function `run`.
6. Documentation Comments: Functions are documented using `///` comments, which can be used to generate documentation.
7. Additional Examples: The `multiply` and `is_even` functions demonstrate how functions can be defined and used to perform specific tasks.
8. Higher-Order Functions: The `additional_example` function shows how closures can be used with higher-order functions like `map` and `filter`.
*/