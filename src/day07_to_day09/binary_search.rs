use std::io;

fn binary_search(arr: &[i32], x: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == x {
            return mid as i32;
        }

        if arr[mid] < x {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    -1
}

pub fn run() {
    let arr = [2, 3, 4, 10, 40, 50, 60, 70, 80, 90];
    println!("Enter the value of x to search for:");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Error reading x value");

    let x: i32 = x.trim().parse()
        .expect("Error converting x value to a number");

    let result = binary_search(&arr, x);
    if result == -1 {
        println!("Element does not exist in the array");
    } else {
        println!("Element found at index: {}", result);
    }
}