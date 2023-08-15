


## COMPILE ERROR

```rust
trait Transform {
    fn transform(&self) -> String;
}

struct Integer(i32);
struct Float(f64);

impl Transform for Integer {
    fn transform(&self) -> String {
        format!("Integer: {}", self.0)
    }
}

impl Transform for Float {
    fn transform(&self) -> String {
        format!("Float: {}", self.0)
    }
}

// Higher-Rank Trait Bound (HRTB) function
fn print_transforms<T>(items: &[T])
where
    for<'a> &'a T: Transform,
{
    for item in items {
        println!("{}", item.transform());
    }
}

fn main() {
    let integers = vec![Integer(42), Integer(99)];
    let floats = vec![Float(3.14), Float(2.718)];

                    // error
    print_transforms(integers);
                    // error
    print_transforms(floats);
}
```