pub fn examples() {
    println!("Iterators Examples:");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    let sum: i32 = numbers.iter().sum();
    println!("Sum of numbers: {}", sum);
}