


```rust
use std::cmp::PartialOrd;
use std::marker::PhantomData;

struct Point<T> {
    x: T,
    y: T,
    _phantom: PhantomData<T>,
}

impl<T: Default + std::fmt::Display> Point<T> {
    fn new(x: T, y: T) -> Self
    where
        T: PartialOrd,
    {
        println!("{}", x);
        if x.partial_cmp(&T::default()).unwrap() == std::cmp::Ordering::Less {
            panic!("x must be non-negative");
        }

        Self {
            x,
            y,
            _phantom: PhantomData,
        }
    }
}

fn main() {
    let point = Point::new(3.0, 4.0);
    println!("Point: x = {}, y = {}", point.x, point.y);

    // Uncommenting the line below will result in a runtime panic
    // let negative_point = Point::new(-2.0, 5.0);
}
```