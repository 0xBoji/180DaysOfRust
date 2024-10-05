// Primitive Types in Rust
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Compound Types:
// Tuples
// Arrays

pub fn run() {
    // Integer Types
    let x: i32 = 1;  // 32-bit signed integer (default integer type)
    let y: u64 = 100;  // 64-bit unsigned integer

    // Floating-Point Types
    let z: f64 = 2.5;  // 64-bit float (default float type)
    let w: f32 = 3.7;  // 32-bit float

    // Finding Maximum Values for Integer Types
    println!("Max i32: {}", std::i32::MAX);
    println!("Max u64: {}", std::u64::MAX);

    // Boolean Type
    let is_active: bool = true;
    let is_greater: bool = 10 > 5;

    // Character Type
    let a1: char = 'a';  // Regular ASCII
    let face: char = '\u{1F600}';  // Unicode emoji

    // Tuple Type (can contain different types)
    let tuple: (i32, f64, char, bool) = (x, z, a1, is_active);

    // Array Type (fixed length, same type)
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // Print examples of each type
    println!("Integer (i32): {}", x);
    println!("Unsigned Integer (u64): {}", y);
    println!("Float (f64): {}", z);
    println!("Float (f32): {}", w);
    println!("Boolean: {}", is_active);
    println!("Boolean from expression: {}", is_greater);
    println!("Character: {}", a1);
    println!("Unicode Character: {}", face);
    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);
}

/*
Key Points:
1. Integer Types: Rust provides both signed (i8, i16, i32, i64, i128) and unsigned (u8, u16, u32, u64, u128) integers. 
   The number indicates the bits used in memory. Default is i32.

2. Floating-Point Types: f32 (32-bit) and f64 (64-bit). Default is f64.

3. Boolean: The bool type represents a value of either true or false.

4. Character: The char type is a Unicode scalar value, always 4 bytes in size.

5. Tuples: A compound type that can group multiple types. Elements accessed by index.

6. Arrays: Fixed-length collections of elements of the same type. Size is part of the type signature.

7. Type Inference: Rust can often infer types, but explicit type annotations improve readability and prevent errors.

8. Memory Safety: Rust's type system and ownership rules prevent many common programming errors at compile-time.
*/