// Arrays - Fixed-size collections of elements with the same data type
use std::mem;

pub fn run() {
    // Declaring an array with explicit type annotation
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Modifying an array element
    numbers[2] = 20;

    // Demonstrating various operations on arrays
    array_operations(&mut numbers);

    // Creating and working with array slices
    demonstrate_slices(&numbers);
}

fn array_operations(numbers: &mut [i32; 5]) {
    println!("Array Operations:");
    
    // Displaying the entire array
    println!("Full array: {:?}", numbers);

    // Accessing a single value
    println!("First element: {}", numbers[0]);

    // Getting array length
    println!("Array length: {}", numbers.len());

    // Calculating memory size of the array
    println!("Memory used: {} bytes", mem::size_of_val(numbers));
}

fn demonstrate_slices(numbers: &[i32; 5]) {
    println!("\nSlice Operations:");
    
    // Creating a slice of the first two elements
    let slice: &[i32] = &numbers[0..2];
    println!("Slice of first two elements: {:?}", slice);

    // Creating a slice of the last three elements
    let slice_end: &[i32] = &numbers[2..];
    println!("Slice of last three elements: {:?}", slice_end);
}

/*
Key Points:
1. Fixed-Size: Arrays in Rust have a fixed size determined at compile time.
2. Type Homogeneity: All elements in an array must be of the same type.
3. Stack Allocation: Arrays are stack-allocated by default, ensuring fast access.
4. Zero-Based Indexing: Array indices start at 0.
5. Bounds Checking: Rust performs bounds checking at runtime to prevent buffer overflows.
6. Slices: Provide a view into a contiguous sequence of elements in the array.
7. Performance: Arrays offer excellent performance for fixed-size data.
8. Mutability: Arrays can be mutable or immutable, controlled by the `mut` keyword.
*/