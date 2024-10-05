// Ownership in Rust

pub fn run() {
    // Create a String on the heap
    let my_banhmi = String::from("bánh mì");
    
    // Ownership of my_banhmi is moved to the eat_food function
    eat_food(my_banhmi);
    
    // Uncommenting the line below would cause a compile error
    // println!("My bánh mì: {}", my_banhmi);  // Error: my_banhmi has been moved
}

fn eat_food(food: String) {
    // The food parameter now owns the String
    println!("I am eating {}!", food);
    // When this function ends, food goes out of scope and is dropped
}