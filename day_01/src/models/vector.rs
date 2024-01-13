// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    // Creating a Vector: Initialize a vector with integers.
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Modifying an Element: Change the value at the third position.
    numbers[2] = 20;

    // Adding Elements: Push new elements to the end of the vector.
    numbers.push(5);
    numbers.push(6);

    // Removing the Last Element: Pop the last element off the vector.
    numbers.pop();

    // Displaying the Vector
    println!("{:?}", numbers);

    // Accessing a Single Value: Access the first element in the vector.
    println!("Single Value: {}", numbers[0]);

    // Getting Vector Length: Display the number of elements in the vector.
    println!("Vector Length: {}", numbers.len());

    // Memory Usage: Display the memory size occupied by the vector.
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Slicing a Vector: Create a slice of the vector.
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Iterating Over Values: Iterate through the vector and print each value.
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Iterating and Modifying: Iterate and modify each element in the vector.
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}

// Additional Example: Demonstrating more vector operations.
pub fn vector_operations() {
    let mut nums = vec![10, 20, 30, 40, 50];

    // Removing an Element at a Specific Index
    nums.remove(1); // Removes 20

    // Inserting an Element at a Specific Index
    nums.insert(1, 25);

    // Extending a Vector with Another
    let other_nums = vec![60, 70, 80];
    nums.extend(other_nums);

    // Filtering Vector Values
    let filtered_nums: Vec<i32> = nums.into_iter().filter(|&x| x > 30).collect();
    println!("Filtered Nums: {:?}", filtered_nums);
}

// Example Usage
pub fn additional_example() {
    run();
    vector_operations();
}
/* 
Key Points:
Creating and Modifying Vectors: Vectors are initialized with Vec<T> and can be modified by adding (push) or removing (pop) elements.
Indexing: Access elements by their index, like numbers[0] for the first element. Remember, Rust uses zero-based indexing.
Dynamic Size: Unlike arrays, vectors can grow or shrink in size.
Memory Allocation: Vectors are allocated on the heap, which allows them to be resized.
Slicing: Create a slice of a vector to work with a portion of its elements.
Iterating Over Vectors: The iter() method is used to iterate over each element, and iter_mut() allows modifying each element in place.
Additional Vector Operations: The vector_operations function shows more complex operations like removing or inserting elements at specific indices, extending a vector with another, and filtering vector elements based on a condition.
*/