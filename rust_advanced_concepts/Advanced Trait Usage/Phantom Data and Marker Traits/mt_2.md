Marker traits in Rust are traits that don't require any methods to be implemented; they are used to provide additional information about types that implement them. Here's an example of how you can define and use marker traits in Rust:

```rust
// Define a marker trait
trait Printable {}

// Implement the marker trait for a type
struct Book;
struct Car;

impl Printable for Book {}
impl Printable for Car {}

// Function that uses the marker trait
fn print_if_printable(item: impl Printable) {
    println!("Printing something...");
}

// We also can write with trait bound:
// fn print_if_printable<T: Printable>(item: T) {
//     println!("Printing something...");
// }


fn main() {
    let book = Book;
    let car = Car;

    print_if_printable(book);
    print_if_printable(car);
}

```

In this example, the `Printable` trait is a marker trait because it doesn't have any methods associated with it. The `Book` and `Car` structs implement the `Printable` trait, providing the information that they can be printed. The `print_if_printable` function accepts any type that implements the `Printable` trait as an argument.

When you run this code, it will print:

```
Printing something...
Printing something...
```

Remember that marker traits are often used to communicate additional information to the compiler or other developers without requiring any specific behavior to be implemented.