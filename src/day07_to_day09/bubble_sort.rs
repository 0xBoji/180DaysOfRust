use std::io;

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn run() {  
    println!("Enter integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");

    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Error converting to number"))
        .collect();

    println!("Array before sorting: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Array after sorting: {:?}", numbers);
}