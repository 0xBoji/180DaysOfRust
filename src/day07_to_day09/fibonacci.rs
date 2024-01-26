fn fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = Vec::with_capacity(n as usize);

    for i in 0..n {
        if i == 0 {
            sequence.push(0);
        } else if i == 1 {
            sequence.push(1);
        } else {
            let last = sequence[i as usize - 1];
            let second_last = sequence[i as usize - 2];
            sequence.push(last + second_last);
        }
    }

    sequence
}

pub fn run() {
    let n = 10;
    let fib_sequence = fibonacci(n);
    println!("First {} Fibonacci numbers: {:?}", n, fib_sequence);
}
