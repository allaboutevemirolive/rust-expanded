trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Polymorphism
// Using &dyn Trait (Trait Object)
fn process_shape_trait(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

// Monomorphization
// Using impl Trait (Opaque Types)
fn process_shape_opaque(shape: &impl Shape) {
    println!("Area: {}", shape.area());
}

// Monomorphization
// Using generics for monomorphization
fn process_shape_generic<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    // Using &dyn Trait (Trait Object)
    process_shape_trait(&circle);
    process_shape_trait(&rectangle);

    // Using impl Trait (Opaque Types)
    process_shape_opaque(&circle);
    process_shape_opaque(&rectangle);

    process_shape_generic(&circle);     // Monomorphized version for Circle
    process_shape_generic(&rectangle);  // Monomorphized version for Rectangle

}
