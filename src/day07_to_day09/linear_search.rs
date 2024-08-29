use std::io;

fn linear_search(arr: &[i32], x: i32) -> i32 {
    for (index, &item) in arr.iter().enumerate() {
        if item == x {
            return index as i32;
        }
    }
    -1
}

pub fn run() {
    let arr = [2, 3, 4, 10, 40];
    println!("Enter the value of x to search for:");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Error reading x value");

    let x: i32 = x.trim().parse()
        .expect("Error converting x value to a number");

    let result = linear_search(&arr, x);
    if result == -1 {
        println!("Element does not exist in the array");
    } else {
        println!("Element found at index: {}", result);
    }
}