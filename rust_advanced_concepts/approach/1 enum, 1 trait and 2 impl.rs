// Define an enum called 'Shape' with different variants
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

// Define a trait called 'CalculateArea' with a method 'calculate_area'
trait CalculateArea {
    fn calculate_area(&self) -> f64;
}

// Implement the 'CalculateArea' trait for the 'Shape' enum
impl CalculateArea for Shape {
    fn calculate_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(length, width) => length * width,
        }
    }
}

// Implement a custom method for the 'Shape' enum
impl Shape {
    fn display(&self) {
        match self {
            Shape::Circle(radius) => println!("Circle with radius: {}", radius),
            Shape::Square(side) => println!("Square with side length: {}", side),
            Shape::Rectangle(length, width) => {
                println!("Rectangle with length: {} and width: {}", length, width)
            }
        }
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(6.0, 8.0);

    circle.display();
    println!("Area of circle: {}", circle.calculate_area());

    square.display();
    println!("Area of square: {}", square.calculate_area());

    rectangle.display();
    println!("Area of rectangle: {}", rectangle.calculate_area());
}
