// Structs - Used to create custom data types

// Traditional Struct: Define a struct with named fields.
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct: Similar to traditional structs, but the fields have no names.
struct Point(f64, f64, f64);

// Defining a Person Struct with named fields.
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// Implementing methods for the Person struct.
impl Person {
    // Constructor Method: Create a new Person instance.
    fn new(first: &str, last: &str, age: u8) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age,
        }
    }

    // Method to Get Full Name: Returns a formatted string with the full name.
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Method to Set Last Name: Modifies the last name of the person.
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Method to have a birthday: Increments the age by 1.
    fn have_birthday(&mut self) {
        self.age += 1;
    }
}

pub fn run() {
    // Using the Color Struct
    let mut sky_color = Color {
        red: 135,
        green: 206,
        blue: 235,
    };
    sky_color.blue = 240;
    println!("Sky Color: RGB({}, {}, {})", sky_color.red, sky_color.green, sky_color.blue);

    // Using the Point Tuple Struct
    let origin = Point(0.0, 0.0, 0.0);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Using the Person Struct
    let mut person = Person::new("Alice", "Johnson", 30);
    println!("Person: {} (Age: {})", person.full_name(), person.age);
    person.set_last_name("Smith");
    person.have_birthday();
    println!("Updated Person: {:?}", person);
}

// Additional Example: Demonstrating a more complex struct with an associated function.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method to Calculate Area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Associated Function to Create a Square
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // Method to check if it's a square
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

pub fn rectangle_example() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle: {:?}", rect);
    println!("Area of the rectangle: {}", rect.area());
    println!("Is it a square? {}", rect.is_square());

    let sq = Rectangle::square(10);
    println!("Square: {:?}", sq);
    println!("Area of the square: {}", sq.area());
    println!("Is it a square? {}", sq.is_square());
}

/* 
Key Points:
1. Traditional Structs: Defined with named fields (e.g., Color), allowing access to each field by name.
2. Tuple Structs: Fields are accessed by index instead of name (e.g., Point).
3. Derive Attributes: #[derive(Debug)] allows for easy printing of struct instances.
4. Methods and Associated Functions: 
   - Methods borrow the struct instance (&self or &mut self).
   - Associated functions (like new or square) don't take self as a parameter.
5. Mutability: Modifying a struct requires it to be mutable (mut).
6. Struct Update Syntax: Can be used to create new instances based on existing ones.
7. Lifetimes: Not shown here, but important for structs that hold references.
8. Unit-like Structs: Structs without any fields are possible and useful in certain scenarios.
*/