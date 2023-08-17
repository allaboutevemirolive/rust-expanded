use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Display: {}", person);
    println!("Debug: {:?}", person);
}
