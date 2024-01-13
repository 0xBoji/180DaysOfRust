// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive Array: Stacks-allocated, fixed-size, and has value semantics.
    let arr1 = [1, 2, 3];
    // Array Copy: When you assign arr1 to arr2, it makes a copy of the values because arrays of primitive types implement the Copy trait.
    let arr2 = arr1;

    // With non-primitives, Rust prevents accidental data duplication.
    // Vector: Heap-allocated, resizable, and has reference semantics.
    let vec1 = vec![1, 2, 3];
    // Borrowing: vec2 holds a reference (&) to vec1. No new vector is created.
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

    // Additional Example: Demonstrating ownership and borrowing.
    show_ownership_and_borrowing();
}

// Function to Show Ownership and Borrowing Concepts
pub fn show_ownership_and_borrowing() {
    let s1 = String::from("hello");
    // Ownership Transfer: s1's ownership is moved to s2. s1 is no longer valid.
    let s2 = s1;
    // println!("{}", s1); // This line would cause a compile-time error because s1's value was moved.

    let s3 = s2.clone(); // Cloning: Creates a deep copy of s2, including data on the heap.
    println!("s2 is {}, s3 is {}", s2, s3); // Both s2 and s3 are valid here.

    let s4 = &s3; // Borrowing: s4 is a reference to s3.
    println!("s3 is {}, s4 is {}", s3, s4); // s3 and s4 can be used here; s4 does not take ownership of s3.
}

/* 
Key Points:
Stack vs Heap: Primitive data types like arrays of integers are stored on the stack and exhibit value semantics, meaning they are copied on assignment. Heap-allocated types like Vec or String exhibit reference semantics.
Ownership in Rust: Rust enforces a unique ownership system to manage memory. When you assign a non-primitive value to another variable, the ownership of that data is transferred to the new variable, and the original variable becomes invalid (unless the data type implements the Copy trait).
Borrowing: Using a reference (&) allows you to borrow a value without taking ownership of it. This concept is crucial for working with complex data structures without unnecessary data duplication.
Cloning: If you do need to create a deep copy of a heap-allocated data structure (like a String or a Vec), you can use the .clone() method.
Function show_ownership_and_borrowing: Demonstrates how ownership is transferred and how data can be borrowed or cloned in Rust.
*/