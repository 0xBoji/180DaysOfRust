mod advanced_traits;
mod advanced_lifetimes;
mod unsafe_rust;
mod macros;
mod advanced_concurrency;

pub fn run() {
    println!("Days 13-15: Advanced Rust Concepts");
    advanced_traits::examples();
    advanced_lifetimes::examples();
    unsafe_rust::examples();
    macros::examples();
    advanced_concurrency::examples();
}