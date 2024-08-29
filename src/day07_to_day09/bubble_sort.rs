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
    println!("Nhập các số nguyên, cách nhau bởi dấu cách:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Lỗi khi đọc input");

    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Lỗi khi chuyển đổi thành số"))
        .collect();

    println!("Mảng trước khi sắp xếp: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("Mảng sau khi sắp xếp: {:?}", numbers);
}
