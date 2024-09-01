pub mod shadowing;
pub mod ownership;
pub mod borrowing;

pub fn run() {
    println!("Days 4-6: Understanding Ownership");
    shadowing::run();
    ownership::run();
    borrowing::run();
}
