use std::io;
use std::cmp::Ordering;

/// Performs a binary search on a sorted array.
///
/// # Arguments
///
/// * `arr` - A sorted slice of integers to search through.
/// * `x` - The value to search for.
///
/// # Returns
///
/// Returns the index of the found element, or -1 if not found.
fn binary_search(arr: &[i32], x: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        match x.cmp(&arr[mid]) {
            Ordering::Equal => return mid as i32,
            Ordering::Greater => left = mid + 1,
            Ordering::Less => right = mid - 1,
        }
    }

    -1
}

/// Runs the binary search demonstration.
pub fn run() {
    let arr = [2, 3, 4, 10, 40, 50, 60, 70, 80, 90];
    println!("Binary Search Demonstration");
    println!("Array: {:?}", arr);
    println!("Enter the value to search for:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let x: i32 = input.trim().parse()
        .expect("Please enter a valid integer");

    let result = binary_search(&arr, x);
    match result {
        -1 => println!("Element {} was not found in the array", x),
        i => println!("Element {} was found at index {}", x, i),
    }
}