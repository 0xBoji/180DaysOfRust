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
    println!("Nhập giá trị x cần tìm kiếm:");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Lỗi khi đọc giá trị x");

    let x: i32 = x.trim().parse()
        .expect("Lỗi khi chuyển đổi giá trị x thành số");

    let result = binary_search(&arr, x);
    if result == -1 {
        println!("Phần tử không tồn tại trong mảng");
    } else {
        println!("Phần tử được tìm thấy tại chỉ số: {}", result);
    }
}