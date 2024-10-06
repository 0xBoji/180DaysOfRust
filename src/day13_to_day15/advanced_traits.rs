use std::ops::Add;
use std::fmt::Debug;

pub fn examples() {
    println!("Advanced Traits Examples:");
    
    associated_types_example();
    default_type_parameters_example();
    fully_qualified_syntax_example();
}

// Example 1: Associated Types
fn associated_types_example() {
    println!("\nAssociated Types Example:");
    let point1 = Point { x: 1, y: 0 };
    let point2 = Point { x: 2, y: 3 };
    let sum = point1 + point2;
    println!("Sum of points: {:?}", sum);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Example 2: Default Type Parameters
fn default_type_parameters_example() {
    println!("\nDefault Type Parameters Example:");
    let millimeters = Millimeters(100);
    let meters = Meters(1);
    let sum = millimeters + meters;
    println!("Sum of lengths: {:?}", sum);
}

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Example 3: Fully Qualified Syntax
fn fully_qualified_syntax_example() {
    println!("\nFully Qualified Syntax Example:");
    let human = Human;
    human.fly();
    Pilot::fly(&human);
    Wizard::fly(&human);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    #[allow(dead_code)]
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}