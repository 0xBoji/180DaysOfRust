use std::io;

fn linear_search(arr: &[i32], x: i32) -> i32 {
    for (index, &item) in arr.iter().enumerate() {
        if item == x {
            return index as i32;
        }
    }
}

pub fn run() {
    let arr = [2, 3, 4, 10, 40];
    println!("Nhập giá trị x cần tìm kếm:");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Lỗi khi đọc giá trị x");

    let x: i32 = x.trim().parse()
        .expect("Lỗi khi chuyển đổi giá trị x thành số");

    let result = linear_search(&arr, x);
    if result == -1 {
        println!("Phần tử không tồn tại trong mảng");
    } else {
        println!("Phần tử được tìm thấy tại chỉ số: {}", result);
    }
}