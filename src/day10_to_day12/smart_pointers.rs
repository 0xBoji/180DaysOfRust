use std::rc::Rc;
use std::cell::RefCell;

pub fn examples() {
    println!("Smart Pointers Examples:");
    
    box_example();
    rc_example();
    refcell_example();
}

fn box_example() {
    // Box<T>: Used for heap allocation
    let boxed_value = Box::new(5);
    println!("Boxed value: {}", boxed_value);
    // When boxed_value goes out of scope, the heap memory is automatically freed
}

fn rc_example() {
    // Rc<T>: Reference Counted smart pointer
    let shared_value = Rc::new(10);
    let cloned_value = Rc::clone(&shared_value);
    println!("Shared value: {}, Cloned value: {}", shared_value, cloned_value);
    println!("Reference count: {}", Rc::strong_count(&shared_value));
    // Memory is freed when the last reference is dropped
}

fn refcell_example() {
    // RefCell<T>: Allows mutable borrows checked at runtime
    let mutable_value = RefCell::new(15);
    {
        let mut borrowed_value = mutable_value.borrow_mut();
        *borrowed_value += 5;
    } // mutable borrow is dropped here
    println!("Mutable value: {}", mutable_value.borrow());
}