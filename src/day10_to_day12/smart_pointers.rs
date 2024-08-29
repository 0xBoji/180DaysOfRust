use std::rc::Rc;
use std::cell::RefCell;

pub fn examples() {
    println!("Smart Pointers Examples:");
    
    // Box<T>
    let boxed_value = Box::new(5);
    println!("Boxed value: {}", boxed_value);

    // Rc<T>
    let shared_value = Rc::new(10);
    let cloned_value = Rc::clone(&shared_value);
    println!("Shared value: {}, Cloned value: {}", shared_value, cloned_value);

    // RefCell<T>
    let mutable_value = RefCell::new(15);
    *mutable_value.borrow_mut() += 5;
    println!("Mutable value: {}", mutable_value.borrow());
}