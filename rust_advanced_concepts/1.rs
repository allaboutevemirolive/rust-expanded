trait Printable {
    fn print_info(&self);
}

struct Person {
    name: String,
    age: u32,
}

struct Car {
    brand: String,
    year: u32,
}

impl Printable for Person {
    fn print_info(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

impl Printable for Car {
    fn print_info(&self) {
        println!("Brand: {}, Year: {}", self.brand, self.year);
    }
}

impl Printable for &str {
    fn print_info(&self) {
        println!("String: {}", *self);
    }
}


// This is a new trait, not Printable
trait DefaultPrintable {
    fn default_print(&self) {
        println!("Information not available.");
    }
}

impl<T: ?Sized + Printable> DefaultPrintable for T {}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    person.print_info();

    let car = Car {
        brand: String::from("Toyota"),
        year: 2023,
    };
    car.print_info();

    let unknown = "Some data";
    unknown.default_print(); // This will use the default implementation
}
