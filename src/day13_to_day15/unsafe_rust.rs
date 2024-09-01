pub fn examples() {
    println!("Unsafe Rust Examples:");
    
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        
        *r2 = 10;
        println!("r2 is now: {}", *r2);
    }
    
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {
    println!("This function is unsafe");
}