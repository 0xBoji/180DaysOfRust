pub fn examples() {
    println!("Closures Examples:");
    
    // Example 1: Simple closure
    let add_one = |x: i32| x + 1;
    println!("Add one to 5: {}", add_one(5));

    // Example 2: Closure with filter
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);

    // Example 3: Closure with map
    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared numbers: {:?}", squared_numbers);

    // Example 4: Closure that captures environment
    let factor = 2;
    let multiply = |x: i32| x * factor;
    println!("Multiply 5 by factor 2: {}", multiply(5));

    // Example 5: Closure as function parameter
    fn apply_operation<F>(x: i32, f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(x)
    }
    let result = apply_operation(3, |x| x * x);
    println!("Result of applying closure to 3: {}", result);
}