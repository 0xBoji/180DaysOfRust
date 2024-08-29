pub fn examples() {
    println!("Closures Examples:");
    
    let add_one = |x| x + 1;
    println!("Add one to 5: {}", add_one(5));

    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);
}