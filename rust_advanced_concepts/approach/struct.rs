// Define a complex number struct
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    // Constructor for Complex struct
    fn new(real: f64, imaginary: f64) -> Self {
        Complex { real, imaginary }
    }

    // Method to add two complex numbers
    fn add(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real + other.real, 
            self.imaginary + other.imaginary
        )
    }

    // Method to multiply two complex numbers
    fn multiply(&self, other: &Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imaginary * other.imaginary,
            self.real * other.imaginary + self.imaginary * other.real,
        )
    }
}

fn main() {
    // Create two complex numbers
    let complex1 = Complex::new(3.0, 4.0);
    let complex2 = Complex::new(1.0, 2.0);

    // Add two complex numbers
    let sum = complex1.add(&complex2);
    println!("Sum: {} + {}i", sum.real, sum.imaginary);

    // Multiply two complex numbers
    let product = complex1.multiply(&complex2);
    println!("Product: {} + {}i", product.real, product.imaginary);

    // Using tuples to represent points in 2D space
    let point = (5, 10);
    println!("Point: ({}, {})", point.0, point.1);
}
