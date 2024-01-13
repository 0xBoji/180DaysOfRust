// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    // Declaring an Array: Here, 'numbers' is an array of integers (i32) with a fixed size of 5.
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assigning Value: Changing the value at index 2 (third element) of the array to 20.
    numbers[2] = 20;

    // Printing the Entire Array: Shows the contents of the array after modification.
    println!("{:?}", numbers);

    // Accessing a Single Value: Prints the first element of the array (index 0).
    println!("Single Value: {}", numbers[0]);

    // Getting Array Length: Displays the number of elements in the array.
    println!("Array Length: {}", numbers.len());

    // Memory Size of the Array: Shows the total size in bytes that the array occupies in the stack.
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Creating a Slice: 'slice' is a reference to a portion of the array, specifically the first two elements.
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
/*Key point:
Key Points:
Fixed-Size Arrays: In Rust, arrays have a fixed size, specified at the time of declaration (e.g., [i32; 5] is an array of five 32-bit integers).
Mutable Arrays: By using mut, the array numbers is made mutable, allowing modification of its elements.
Indexing: Arrays use zero-based indexing. Accessing or modifying elements is done using their index (e.g., numbers[2] refers to the third element).
Array Length: The .len() method returns the number of elements in the array.
Stack Allocation: Arrays in Rust are stack-allocated (unless they're boxed), and mem::size_of_val is used to determine the amount of stack memory used by the array.
Slices: A slice is a view into a contiguous sequence of elements in a collection (like an array). Slices are denoted by [start..end], where start is inclusive and end is exclusive. In your code, &numbers[0..2] creates a slice containing the first two elements of the numbers array.
*/