// Define the Shape trait
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

// Circle struct and implementation
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &'static str {
        "Circle"
    }
}

// Rectangle struct and implementation
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &'static str {
        "Rectangle"
    }
}

// Function to print shape information
fn print_shape_info(shape: &dyn Shape) {
    println!("{} area: {:.2}", shape.name(), shape.area());
}

pub fn examples() {
    println!("Traits Examples:");
    
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };

    print_shape_info(&circle);
    print_shape_info(&rectangle);

    // Additional example with a vector of shapes
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 2.0, height: 4.0 }),
    ];

    for shape in shapes.iter() {
        print_shape_info(&**shape);
    }
}