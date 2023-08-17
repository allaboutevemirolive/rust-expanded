

Dynamic dispatch in Rust refers to the process of selecting the appropriate implementation of a function or method at runtime, based on the actual type of the object being operated on. This is in contrast to static dispatch, where the implementation is determined at compile time.

In Rust, dynamic dispatch is typically achieved through the use of trait objects and the `dyn` keyword. Here's how it works:

1. **Traits**: In Rust, traits define a set of methods that types can implement. They are similar to interfaces in other languages. For example, let's define a simple trait called `Shape`:

```rust
trait Shape {
    fn area(&self) -> f64;
}
```

2. **Structs and Implementations**: Types can then implement this trait by providing the necessary implementations for its methods. For instance, we can define two structs that implement the `Shape` trait:

```rust
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

3. **Dynamic Dispatch with Trait Objects**: The key concept behind dynamic dispatch is using trait objects. A trait object is a reference or a pointer to a value of a type that implements a trait. This allows you to store multiple different types that implement the same trait in a single data structure.

```rust
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}
```

In this example, the `print_area` function takes a reference to a trait object that implements the `Shape` trait. This means it can accept either a `Circle` or a `Rectangle`, or any other type that implements `Shape`.

4. **Use Case - Polymorphism and Code Reuse**: Dynamic dispatch is particularly useful when you want to write code that works with different types that share a common behavior, but you want to decide which implementation to use at runtime. This is a form of polymorphism, where different types can be treated uniformly based on their shared traits.

For instance, you might have a scenario where you want to calculate the areas of various shapes without knowing their exact types at compile time:

```rust
fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];

    for shape in shapes {
        print_area(shape);
    }
}
```

Here, the `print_area` function is dynamically dispatched based on the actual type of the shape at runtime. This allows you to write generic code that operates on different types implementing the same trait, promoting code reuse and maintainability.

However, it's important to note that dynamic dispatch comes with some runtime performance overhead compared to static dispatch. Rust's ownership and borrowing rules help ensure safety even in the presence of dynamic dispatch, but you should be aware of the trade-offs when choosing between static and dynamic dispatch for your specific use case.




## Code example for `Polymorphism`



```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

enum AnyShape<'a> {
    Circle(&'a Circle),
    Rectangle(&'a Rectangle),
}

// In dynamic dispatch and trait objects, when we pass a
// value as a reference (&) to a function
// that accepts a trait object, we're actually passing a
// reference to the trait object, not to the original value

fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    let shapes: Vec<AnyShape> = vec![AnyShape::Circle(&circle), AnyShape::Rectangle(&rectangle)];

    for shape in shapes {
        match shape {
            AnyShape::Circle(circle) => print_area(circle),
            AnyShape::Rectangle(rectangle) => print_area(rectangle),
        }
    }
}
```