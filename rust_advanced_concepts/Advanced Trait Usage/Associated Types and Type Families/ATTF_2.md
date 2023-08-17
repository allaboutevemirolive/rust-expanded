


## With associated types

```rust
trait Shape {
    type Area;

    fn calculate_area(&self) -> Self::Area;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    type Area = f64;

    fn calculate_area(&self) -> Self::Area {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    type Area = f64;

    fn calculate_area(&self) -> Self::Area {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };

    println!("Circle area: {}", circle.calculate_area()); // Output: Circle area: 28.274333882308138
    println!("Rectangle area: {}", rectangle.calculate_area()); // Output: Rectangle area: 20
}
```



## Without asscociated types

```rust
trait Shape {
    fn calculate_area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };

    println!("Circle area: {}", circle.calculate_area()); // Output: Circle area: 28.274333882308138
    println!("Rectangle area: {}", rectangle.calculate_area()); // Output: Rectangle area: 20
}
```
