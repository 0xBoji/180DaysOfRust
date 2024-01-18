// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite Loop: This loop will run indefinitely until it encounters a 'break'.
    // Uncomment to use.
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     // Breaking the Loop: The loop stops when count reaches 20.
    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop: Executes as long as the condition (count <= 100) remains true.
    // Uncomment to use.
    // while count <= 100 {
    //     // FizzBuzz Logic: Prints "fizzbuzz", "fizz", "buzz", or the number.
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz")
    //     } else {
    //         println!("{}", count);
    //     }

    //     // Increment Counter
    //     count += 1;
    // }

    // For Loop with Range: Iterates over a range of numbers (0 to 99).
    for x in 0..100 {
        // Applying FizzBuzz logic within the for loop.
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
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
Infinite Loop: Created using the loop keyword. It runs indefinitely until explicitly broken out of, often with a break statement.
While Loop: A while loop continues as long as its condition is true. In your example, it's used to implement the FizzBuzz algorithm up to 100.
For Loop: A for loop is used for iterating over a range or a collection. In your code, a for loop is used for FizzBuzz, iterating from 0 to 99.
FizzBuzz Logic: Your loops implement the classic FizzBuzz problem, where numbers divisible by 3 and 5 are replaced with "fizzbuzz", by 3 only with "fizz", and by 5 only with "buzz".
Iterating Over Collections: The additional example iterate_over_array demonstrates iterating over an array using a for loop and the .iter() method, which is a common Rust pattern for traversing collections.
*/
