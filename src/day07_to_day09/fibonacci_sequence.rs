use std::io;

/// Generates a Fibonacci sequence of the specified length.
///
/// # Arguments
///
/// * `n` - The number of elements to generate in the sequence.
///
/// # Returns
///
/// A vector containing the Fibonacci sequence.
fn fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = Vec::with_capacity(n as usize);
    match n {
        0 => return sequence,
        1 => sequence.push(0),
        _ => {
            sequence.extend_from_slice(&[0, 1]);
            for i in 2..n as usize {
                let next = sequence[i - 1].saturating_add(sequence[i - 2]);
                sequence.push(next);
            }
        }
    }
    sequence
}

/// Prompts the user for input and generates a Fibonacci sequence.
pub fn run() {
    println!("Enter the number of elements in the Fibonacci sequence:");
    let n: u32 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(num) if num > 0 => break num,
            _ => println!("Please enter a positive integer greater than 0:"),
        }
    };

    let fib_sequence = fibonacci(n);
    println!("Fibonacci sequence with {} elements:", n);
    println!("{:?}", fib_sequence);
}