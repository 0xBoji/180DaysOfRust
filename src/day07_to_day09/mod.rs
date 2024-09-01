pub mod fibonacci;
pub mod linear_search;
pub mod binary_search;
pub mod bubble_sort;
pub mod palindrome_checker;
pub mod fibonacci_sequence;

pub fn run() {
    println!("Days 7-9: Common Collections");
    fibonacci_sequence::run();
    linear_search::run();
    binary_search::run();
    bubble_sort::run();
    palindrome_checker::run();
    fibonacci_sequence::run();
}