// Loops - Used to iterate until a condition is met

pub fn run() {
    // Infinite Loop Example
    #[allow(dead_code)]
    fn infinite_loop_example() {
        let mut count = 0;
        loop {
            count += 1;
            println!("Number: {}", count);
            if count == 20 {
                break;
            }
        }
    }

    // While Loop Example with FizzBuzz
    #[allow(dead_code)]
    fn while_loop_fizzbuzz() {
        let mut count = 1;
        while count <= 100 {
            print_fizzbuzz(count);
            count += 1;
        }
    }

    // For Loop Example with FizzBuzz
    fn for_loop_fizzbuzz() {
        for x in 1..=100 {
            print_fizzbuzz(x);
        }
    }

    // Helper function for FizzBuzz logic
    fn print_fizzbuzz(n: i32) {
        match (n % 3 == 0, n % 5 == 0) {
            (true, true) => println!("fizzbuzz"),
            (true, false) => println!("fizz"),
            (false, true) => println!("buzz"),
            (false, false) => println!("{}", n),
        }
    }

    // Uncomment to run examples
    // infinite_loop_example();
    // while_loop_fizzbuzz();
    for_loop_fizzbuzz();
}

// Additional Example: Demonstrating the use of a 'for' loop to iterate over an array.
pub fn iterate_over_array() {
    let numbers = [1, 2, 3, 4, 5];

    // Iterating Over an Array
    for num in numbers.iter() {
        println!("Array element: {}", num);
    }
}

// Example Usage
pub fn additional_example() {
    run();
    iterate_over_array();
}

/* 
Key Points:
1. Infinite Loop: Created using the `loop` keyword. It runs indefinitely until explicitly broken out of, often with a `break` statement.
2. While Loop: Continues as long as its condition is true. Used here to implement FizzBuzz up to 100.
3. For Loop: Used for iterating over a range or a collection. Here, it's used for FizzBuzz from 1 to 100.
4. FizzBuzz Logic: Implemented using pattern matching for cleaner code.
5. Iterating Over Collections: The `iterate_over_array` function demonstrates iterating over an array using a for loop and the `.iter()` method.
6. Function Organization: Each loop type is encapsulated in its own function for better organization and readability.
7. Range Syntax: The for loop now uses `1..=100` to include 100 in the range.
*/
