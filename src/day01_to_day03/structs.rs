// Structs - Used to create custom data types

// Traditional Struct: Define a struct with named fields.
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct: Similar to traditional structs, but the fields have no names.
// struct Color(u8, u8, u8);

// Defining a Person Struct with named fields.
struct Person {
    first_name: String,
    last_name: String,
}

// Implementing methods for the Person struct.
impl Person {
    // Constructor Method: Create a new Person instance.
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
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

    // Convert to Tuple: Returns a tuple containing the first and last names.
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Creating and Using the Color Struct: Uncomment to use.
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // c.red = 200;
    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Creating and Using the Color Tuple Struct: Uncomment to use.
    // let mut c = Color(255, 0, 0);
    // c.0 = 200;
    // println!("Color: {} {} {}", c.0, c.1, c.2);

    // Using the Person Struct
    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}

// Additional Example: Demonstrating a more complex struct with an associated function.
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
}

pub fn rectangle_example() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area of the rectangle: {}", rect.area());

    let sq = Rectangle::square(10);
    println!("Area of the square: {}", sq.area());
}

/* 
Key Points:
Traditional Structs: Defined with named fields (e.g., Color { red: u8, green: u8, blue: u8 }), allowing access to each field by name.
Tuple Structs: Similar to traditional structs but fields are accessed by index instead of name (e.g., Color(255, 0, 0)).
Methods and Associated Functions: impl block allows defining methods (like full_name) and associated functions (like new for Person) for a struct. Methods borrow the struct instance (&self or &mut self), while associated functions do not take self as a parameter.
Mutable Struct: Modifying a struct requires it to be mutable (mut). For example, set_last_name modifies Person.
Rectangle Example: Demonstrates a Rectangle struct with an area calculation method and an associated function to create a square.
*/