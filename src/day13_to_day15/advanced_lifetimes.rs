/// This module demonstrates advanced lifetime concepts in Rust.
/// It covers basic lifetime annotations, lifetimes in structs and impl blocks,
/// and multiple lifetime parameters.
pub fn examples() {
    println!("Advanced Lifetimes Examples:");
    
    // Example 1: Basic lifetime annotation
    basic_lifetime_example();

    // Example 2: Lifetime in structs and impl blocks
    struct_lifetime_example();

    // Example 3: Multiple lifetime parameters
    multiple_lifetime_example();
}

/// Demonstrates a basic use of lifetime annotations.
/// This example shows how lifetimes ensure that references remain valid.
fn basic_lifetime_example() {
    println!("\nBasic Lifetime Example:");
    let x = 5;
    let r = &x;
    print_ref(r);
}

/// A generic function that prints a reference.
/// The lifetime 'a ensures that the reference 't' is valid for the duration of the function call.
fn print_ref<'a, T>(t: &'a T) where T: std::fmt::Debug + 'a {
    println!("t: {:?}", t);
}

/// Demonstrates the use of lifetimes in structs and impl blocks.
/// This example shows how lifetimes can be used to ensure that struct fields
/// remain valid for as long as the struct itself.
fn struct_lifetime_example() {
    println!("\nStruct Lifetime Example:");
    let context = Context("context");
    let parser = Parser { context: &context };
    parser.parse("input");
}

/// A struct that holds a reference to a string slice.
/// The lifetime 'a ensures that the reference remains valid for the lifetime of the struct.
struct Context<'a>(&'a str);

/// A struct that holds a reference to a Context.
/// The lifetime 'a ensures that the Context reference remains valid for the lifetime of the Parser.
struct Parser<'a> {
    context: &'a Context<'a>,
}

/// Implementation block for Parser.
/// The lifetime 'a is carried over from the struct definition.
impl<'a> Parser<'a> {
    /// Parses input using the context.
    /// This method demonstrates how the lifetime 'a allows us to use the context reference safely.
    fn parse(&self, input: &str) {
        println!("Parsing {} in context {}", input, self.context.0);
    }
}

/// Demonstrates the use of multiple lifetime parameters.
/// This example shows how different lifetimes can be used to express more complex borrowing relationships.
fn multiple_lifetime_example() {
    println!("\nMultiple Lifetime Example:");
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest string: {}", result);
    }
}

/// A function that takes two string slices with different lifetimes and returns the longer one.
/// The returned reference is guaranteed to live at least as long as the shorter of 'a and 'b.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}