pub fn examples() {
    println!("Generics Examples:");
    
    // Example with integers
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Largest number: {}", largest(&numbers));

    // Example with characters
    let chars = vec!['a', 'b', 'c', 'd', 'e'];
    println!("Largest char: {}", largest(&chars));

    // Example with floating-point numbers
    let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Largest float: {}", largest(&floats));

    // Example with custom type
    let points = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }, Point { x: 5, y: 6 }];
    println!("Largest point: {:?}", largest(&points));
}

// Generic function to find the largest element in a slice
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    list.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

// Custom type for demonstration
#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}