/// A macro that prints a greeting.
///
/// # Examples
///
/// ```
/// say_hello!(); // Prints: Hello!
/// say_hello!("Alice"); // Prints: Hello, Alice!
/// ```
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

/// A macro that creates a function with a given name.
///
/// The created function will print a message when called.
///
/// # Examples
///
/// ```
/// create_function!(foo);
/// foo(); // Prints: You called "foo"()
/// ```
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Create example functions using the macro
create_function!(foo);
create_function!(bar);

/// Demonstrates the usage of macros defined in this module.
pub fn examples() {
    println!("Macros Examples:");
    
    // Demonstrate say_hello! macro
    say_hello!();
    say_hello!("Alice");
    
    // Demonstrate create_function! macro
    foo();
    bar();
}