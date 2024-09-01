pub mod closures;
pub mod error_handling;
pub mod generics;
pub mod iterators;
pub mod lifetimes;
pub mod smart_pointers;
pub mod traits;

pub fn run() {
    println!("Days 10-12: Advanced Rust Concepts");
    closures::examples();
    error_handling::examples();
    generics::examples();
    iterators::examples();
    lifetimes::examples();
    smart_pointers::examples();
    traits::examples();
}