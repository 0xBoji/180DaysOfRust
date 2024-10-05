pub fn examples() {
    println!("Iterators Examples:");
    
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Demonstrate map() and collect()
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    // Demonstrate sum()
    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);

    // Demonstrate filter() and collect()
    let even_numbers: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);

    // Demonstrate fold()
    let product: i32 = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product of numbers: {}", product);

    // Demonstrate chain() and max()
    let more_numbers = vec![6, 7, 8];
    let max_number = numbers.iter().chain(more_numbers.iter()).max();
    println!("Maximum number: {:?}", max_number);
}