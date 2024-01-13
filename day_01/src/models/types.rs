// Primitive Types
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

pub fn run() {
    // Integer Type: The default is "i32", a 32-bit signed integer.
    let x = 1;

    // Floating-Point Type: The default is "f64", a 64-bit floating-point number.
    let y = 2.5;

    // Explicit Type Annotation: Here, "z" is explicitly annotated as "i64", a 64-bit signed integer.
    let z: i64 = 45454545454545;

    // Finding Maximum Value: Demonstrates how to find the maximum value for integer types.
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean Type: "is_active" is a variable of type "bool" which represents a boolean value.
    let is_active: bool = true;

    // Boolean from Expression: "is_greater" is a boolean obtained from evaluating the expression "10 < 5".
    let is_greater: bool = 10 < 5;

    // Character Type: "a1" is a character type, 'char', representing a single Unicode scalar value.
    let a1 = 'a';

    // Unicode Scalar Values: "face" is a 'char' type representing a Unicode emoji using its code point.
    let face = '\u{1F600}';

    // Tuple: Demonstrates a tuple containing different types.
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}


/*Key point:
Integer Types: Rust provides both signed (i8, i16, i32, i64, i128) and unsigned (u8, u16, u32, u64, u128) integers. The number (e.g., 8, 16, 32) indicates the number of bits used in memory.
Floating-Point Types: There are two floating-point types, f32 and f64, corresponding to 32-bit and 64-bit floating-point numbers, respectively.
Boolean: The bool type represents a Boolean value, which can be either true or false.
Character: The char type is a Unicode scalar value ranging from U+0000 to U+D7FF and U+E000 to U+10FFFF.
Tuples: A compound type that can group together a variety of types. Tuple members are accessed by their position.
Arrays: Collections of elements of the same type. Arrays have a fixed length and the elements are stored next to each other in memory.
*/ 