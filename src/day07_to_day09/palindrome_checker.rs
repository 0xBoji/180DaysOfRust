use std::io;

fn is_palindrome(s: &str) -> bool {
    let s: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let s = s.to_lowercase();
    s.chars().eq(s.chars().rev())
}

pub fn run() {
    println!("Nhập chuỗi cần kiểm tra palindrome:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Lỗi khi đọc input");

    let input = input.trim();
    if is_palindrome(input) {
        println!("'{}' là một palindrome.", input);
    } else {
        println!("'{}' không phải là một palindrome.", input);
    }
}
