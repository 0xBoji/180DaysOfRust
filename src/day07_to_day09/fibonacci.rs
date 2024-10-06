/// Calculates the Fibonacci sequence up to the nth number.
///
/// The Fibonacci sequence is a series of numbers where each number is the sum of the two preceding ones.
/// It typically starts with 0 and 1, and each subsequent number is the sum of the previous two.
///
/// # Arguments
///
/// * `n` - The number of Fibonacci numbers to generate.
///
/// # Returns
///
/// A vector containing the first `n` numbers in the Fibonacci sequence.
///
/// # Examples
///
/// ```
/// let fib_sequence = fibonacci(5);
/// assert_eq!(fib_sequence, vec![0, 1, 1, 2, 3]);
/// ```
fn fibonacci(n: u32) -> Vec<u32> {
    let mut sequence = Vec::with_capacity(n as usize);
    
    match n {
        0 => return sequence,
        1 => {
            sequence.push(0);
            return sequence;
        }
        _ => {
            sequence.push(0);
            sequence.push(1);
        }
    }

    for i in 2..n as usize {
        let next = sequence[i - 1].saturating_add(sequence[i - 2]);
        sequence.push(next);
    }

    sequence
}

/// Runs the Fibonacci sequence generator and prints the result.
///
/// This function generates the first 10 Fibonacci numbers and prints them to the console.
pub fn run() {
    let n = 10;
    let fib_sequence = fibonacci(n);
    println!("First {} Fibonacci numbers: {:?}", n, fib_sequence);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), Vec::<u32>::new());
        assert_eq!(fibonacci(1), vec![0]);
        assert_eq!(fibonacci(2), vec![0, 1]);
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3]);
        assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
