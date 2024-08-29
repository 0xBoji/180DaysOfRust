pub fn examples() {
    println!("Generics Examples:");
    
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    println!("Largest char: {}", largest(&chars));
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}