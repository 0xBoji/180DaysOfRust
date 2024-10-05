use std::io;

/// Performs a linear search on the given array to find the specified element.
///
/// # Arguments
///
/// * `arr` - A slice of i32 values to search through
/// * `x` - The value to search for
///
/// # Returns
///
/// Returns the index of the found element, or -1 if not found
fn linear_search(arr: &[i32], x: i32) -> Option<usize> {
    arr.iter().position(|&item| item == x)
}

pub fn run() {
    let arr = [2, 3, 4, 10, 40];
    println!("Enter the value to search for:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let x: i32 = input.trim().parse()
        .expect("Invalid input: please enter a number");

    match linear_search(&arr, x) {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element does not exist in the array"),
    }
}