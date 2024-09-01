use std::ops::Add;

pub fn examples() {
    println!("Advanced Traits Examples:");
    
    // Associated types
    let point1 = Point { x: 1, y: 0 };
    let point2 = Point { x: 2, y: 3 };
    let sum = point1 + point2;
    println!("Sum of points: {:?}", sum);

    // Default type parameters
    let millimeters = Millimeters(100);
    let meters = Meters(1);
    let sum = millimeters + meters;
    println!("Sum of lengths: {:?}", sum);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}