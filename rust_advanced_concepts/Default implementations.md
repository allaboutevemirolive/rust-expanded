
```rust
// Define a trait with a default method implementation
trait Greet {
    fn greet(&self);
    
    // Default implementation for the greet method
    fn default_greeting(&self) {
        println!("Hello, default greeting!");
    }
}

// Implement the Greet trait for a custom type
struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

// Implement the Greet trait for another custom type without overriding the default_greeting method
struct Alien;

impl Greet for Alien {
    fn greet(&self) {
        println!("Greetings, extraterrestrial!");
    }
}

fn main() {
    let person = Person { name: "Alice".to_string() };
    let alien = Alien;

    person.greet();  // Calls the custom implementation
    alien.greet();   // Calls the custom implementation
    alien.default_greeting();  // Calls the default implementation
}
```