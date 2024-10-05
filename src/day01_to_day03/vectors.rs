// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    // Creating a Vector: Initialize a vector with integers.
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Modifying an Element: Change the value at the third position (index 2).
    numbers[2] = 20;

    // Adding Elements: Push new elements to the end of the vector.
    numbers.push(5);
    numbers.push(6);

    // Removing the Last Element: Pop the last element off the vector.
    let popped = numbers.pop();
    println!("Popped element: {:?}", popped);

    // Displaying the Vector
    println!("Current vector: {:?}", numbers);

    // Accessing a Single Value: Access the first element in the vector.
    println!("First element: {}", numbers[0]);

    // Getting Vector Length: Display the number of elements in the vector.
    println!("Vector length: {}", numbers.len());

    // Memory Usage: Display the memory size occupied by the vector.
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Slicing a Vector: Create a slice of the vector.
    let slice: &[i32] = &numbers[1..3];
    println!("Slice (elements at index 1 and 2): {:?}", slice);

    // Iterating Over Values: Iterate through the vector and print each value.
    println!("Iterating over vector:");
    for (index, &value) in numbers.iter().enumerate() {
        println!("Element at index {}: {}", index, value);
    }

    // Iterating and Modifying: Iterate and modify each element in the vector.
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Vector after doubling each element: {:?}", numbers);
}

// Additional Example: Demonstrating more vector operations.
pub fn vector_operations() {
    let mut nums = vec![10, 20, 30, 40, 50];
    println!("Initial vector: {:?}", nums);

    // Removing an Element at a Specific Index
    let removed = nums.remove(1); // Removes 20
    println!("Removed element: {}", removed);
    println!("Vector after removal: {:?}", nums);

    // Inserting an Element at a Specific Index
    nums.insert(1, 25);
    println!("Vector after inserting 25 at index 1: {:?}", nums);

    // Extending a Vector with Another
    let other_nums = vec![60, 70, 80];
    nums.extend(other_nums);
    println!("Vector after extending: {:?}", nums);

    // Filtering Vector Values
    let filtered_nums: Vec<i32> = nums.iter().filter(|&&x| x > 30).cloned().collect();
    println!("Filtered vector (elements > 30): {:?}", filtered_nums);

    // Finding an element
    if let Some(index) = nums.iter().position(|&x| x == 40) {
        println!("Found 40 at index: {}", index);
    }

    // Sorting the vector
    nums.sort();
    println!("Sorted vector: {:?}", nums);
}

/* 
Key Points:
1. Creating and Modifying Vectors: Vectors are initialized with Vec<T> and can be modified dynamically.
2. Indexing: Access elements by their index (zero-based), e.g., numbers[0] for the first element.
3. Dynamic Size: Unlike arrays, vectors can grow or shrink in size.
4. Memory Allocation: Vectors are allocated on the heap, allowing for resizing.
5. Slicing: Create a slice of a vector to work with a portion of its elements.
6. Iterating: Use iter() for immutable references, iter_mut() for mutable references.
7. Additional Operations: 
   - remove(): Remove an element at a specific index.
   - insert(): Insert an element at a specific index.
   - extend(): Add all elements from another vector.
   - filter(): Create a new vector with elements that satisfy a condition.
   - position(): Find the index of an element.
   - sort(): Sort the vector in ascending order.
8. Safety: Rust's borrow checker ensures memory safety and prevents data races.
*/