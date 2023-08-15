// Define an enum called 'Vehicle' with different variants
enum Vehicle {
    Car(String),
    Bike(String),
}

// Define a trait called 'Drive' with a method 'start_engine'
trait Drive {
    fn start_engine(&self);
}

// Define another trait called 'Park' with a method 'park_vehicle'
trait Park {
    fn park_vehicle(&self);
}

// Implement the 'Drive' trait for the 'Vehicle' enum
impl Drive for Vehicle {
    fn start_engine(&self) {
        match self {
            Vehicle::Car(name) => println!("Starting the car: {}", name),
            Vehicle::Bike(name) => println!("Starting the bike: {}", name),
        }
    }
}

// Implement the 'Park' trait for the 'Vehicle' enum
impl Park for Vehicle {
    fn park_vehicle(&self) {
        match self {
            Vehicle::Car(name) => println!("Parking the car: {}", name),
            Vehicle::Bike(name) => println!("Parking the bike: {}", name),
        }
    }
}

fn main() {
    let car = Vehicle::Car(String::from("Sedan"));
    let bike = Vehicle::Bike(String::from("Mountain Bike"));

    car.start_engine();
    car.park_vehicle();

    bike.start_engine();
    bike.park_vehicle();
}
