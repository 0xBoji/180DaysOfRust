use std::io;

fn fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();
    if n >= 1 {
        sequence.push(0);
    }
    if n >= 2 {
        sequence.push(1);
    }
    for _ in 2..n {
        let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
        sequence.push(next);
    }
    sequence
}

pub fn run() {
    println!("Nhập số lượng phần tử trong dãy Fibonacci:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Lỗi khi đọc input");

    let n: u32 = input.trim().parse().expect("Vui lòng nhập một số nguyên dương");

    let fib_sequence = fibonacci(n);
    println!("Dãy Fibonacci với {} phần tử: {:?}", n, fib_sequence);
}
