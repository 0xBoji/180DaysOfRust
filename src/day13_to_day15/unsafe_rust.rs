pub fn examples() {
    println!("Unsafe Rust Examples:");
    
    raw_pointers_example();
    unsafe_function_example();
    unsafe_trait_example();
}

fn raw_pointers_example() {
    println!("\nRaw Pointers Example:");
    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        
        *r2 = 10;
        println!("r2 is now: {}", *r2);
    }
}

fn unsafe_function_example() {
    println!("\nUnsafe Function Example:");
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {
    println!("This function is unsafe and could potentially cause undefined behavior");
}

fn unsafe_trait_example() {
    println!("\nUnsafe Trait Example:");
    let mut num = 5;
    let raw = &mut num as *mut i32;
    
    let mut unsafe_wrapper = UnsafeWrapper { ptr: raw };
    
    unsafe {
        unsafe_wrapper.increment();
        println!("Value after increment: {}", *raw);
    }
}

struct UnsafeWrapper {
    ptr: *mut i32,
}

unsafe trait Incrementable {
    unsafe fn increment(&mut self);
}

unsafe impl Incrementable for UnsafeWrapper {
    unsafe fn increment(&mut self) {
        *self.ptr += 1;
    }
}