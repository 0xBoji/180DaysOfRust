// Reference Pointers - Point to a resource in memory

pub fn run() {
    demonstrate_array_behavior();
    demonstrate_vector_behavior();
    show_ownership_and_borrowing();
}

fn demonstrate_array_behavior() {
    // Primitive Array: Stack-allocated, fixed-size, and has value semantics.
    let arr1 = [1, 2, 3];
    // Array Copy: When you assign arr1 to arr2, it makes a copy of the values
    // because arrays of primitive types implement the Copy trait.
    let arr2 = arr1;

    println!("Array values: {:?}", (arr1, arr2));
}

fn demonstrate_vector_behavior() {
    // Vector: Heap-allocated, resizable, and has reference semantics.
    let vec1 = vec![1, 2, 3];
    // Borrowing: vec2 holds a reference (&) to vec1. No new vector is created.
    let vec2 = &vec1;

    println!("Vector values: {:?}", (&vec1, vec2));
}

// Function to Show Ownership and Borrowing Concepts
fn show_ownership_and_borrowing() {
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
Key Concepts:

1. Stack vs Heap:
   - Stack: Used for primitive types (like integer arrays). Fast, fixed size, value semantics.
   - Heap: Used for dynamic types (like Vec or String). Flexible size, reference semantics.

2. Ownership in Rust:
   - Each value has a single owner.
   - When the owner goes out of scope, the value is dropped.
   - Assigning a non-Copy type transfers ownership.

3. Borrowing:
   - References (&) allow borrowing without taking ownership.
   - Multiple immutable borrows or one mutable borrow at a time.

4. Copy vs. Move:
   - Types implementing Copy trait are duplicated on assignment.
   - Other types are moved, transferring ownership.

5. Cloning:
   - .clone() creates a deep copy of heap data.
   - Useful when you need independent copies of data.

These concepts form the foundation of Rust's memory safety guarantees
without needing a garbage collector.
*/