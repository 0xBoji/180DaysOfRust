use std::io;

/// Sorts an array of integers using the bubble sort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable slice of integers to be sorted.
///
/// # Example
///
/// ```
/// let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
/// bubble_sort(&mut numbers);
/// assert_eq!(numbers, vec![11, 12, 22, 25, 34, 64, 90]);
/// ```
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        // If no swapping occurred, array is already sorted
        if !swapped {
            break;
        }
    }
}

/// Runs the bubble sort program, allowing user input and displaying results.
pub fn run() {
    println!("Bubble Sort Algorithm");
    println!("--------------------");
    println!("Enter integers separated by spaces:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut numbers: Vec<i32> = match input
        .split_whitespace()
        .map(|s| s.parse())
        .collect()
    {
        Ok(nums) => nums,
        Err(_) => {
            println!("Error: Invalid input. Please enter valid integers.");
            return;
        }
    };

    println!("\nArray before sorting: {:?}", numbers);
    
    bubble_sort(&mut numbers);
    
    println!("Array after sorting:  {:?}", numbers);
}