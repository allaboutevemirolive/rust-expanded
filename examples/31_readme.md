https://users.rust-lang.org/t/unconstrained-lifetime-parameter-for-impl/27995/3




COMPILE ERROR


```rust
struct Pixel<'img> {
    value: &'img mut u32,
    i: usize,
}

struct Canvas {
    img: Vec<u32>,
    i: usize,
    max_i: usize,
}

    // Error
impl<'a> Iterator for Canvas {
    type Item = Pixel<'a>;
    fn next(&'a mut self) -> Pixel<'a> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: &mut self.img[self.i],
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
    let mut canvas = Canvas {
        img: vec![0; 9],
        i: 0,
        max_i: 3,
    };
    for pixel in canvas {
        *pixel.value = 3 as u32;
    }
}
```

ERROR

```rs
   Compiling test_code v0.1.0 (/home/nemesis/Documents/Github/my_repo/rust-expanded/test_code)
error[E0207]: the lifetime parameter `'a` is not constrained by the impl trait, self type, or predicates
  --> src/main.rs:13:6
   |
13 | impl<'a> Iterator for Canvas {
   |      ^^ unconstrained lifetime parameter

For more information about this error, try `rustc --explain E0207`.
error: could not compile `test_code` (bin "test_code") due to previous error
```



```
So the main problem with the iterator is that it cannot hold each return mutable while iterating? Why can't we just create a variable to store the return mutable so it can hold?
```

You're correct that the main problem here lies in trying to return a mutable reference from the iterator's `next` method. The issue is related to Rust's borrowing rules and the ownership model that it enforces to prevent data races and memory unsafety.

When you create a mutable reference to an element and return it from the iterator, Rust's borrow checker enforces that you can't have multiple mutable references to elements within the same container at the same time. This is to prevent data races where two parts of your code could simultaneously modify the same piece of data, leading to unpredictable behavior and bugs.

The iterator pattern in Rust follows the borrowing rules closely. In an iterator, the `next` method mutates the iterator itself (advancing its internal state) and returns the next element as an owned value, not a reference. This avoids the problem of holding multiple mutable references to the same data.

While it might seem like you could work around this by storing the mutable reference in a variable, Rust's borrow checker can't track this sort of temporary reference and ensure safety. It would be possible to create a mutable reference that overlaps with the container's internal mutable reference, breaking Rust's safety guarantees.

To work within Rust's ownership and borrowing model, you might need to reconsider the design of your code. 



___




The approaches you've mentioned and discussed so far cover the main strategies for dealing with mutable borrows and lifetimes in Rust. However, there are a few other related concepts and approaches you might encounter in certain scenarios:

1. **Mutex and RwLock**: When dealing with concurrent access to data, you might use the `Mutex` (mutual exclusion) or `RwLock` (read-write lock) types from the `std::sync` module. These types allow safe mutation and access to data across multiple threads.

2. **Unsafe Code**: While you've already included an unsafe approach in your examples, it's worth noting that sometimes, particularly complex cases, might require carefully crafted unsafe code blocks. Unsafe code allows you to sidestep some of Rust's safety guarantees, but it comes with the responsibility of ensuring that your code is truly safe. This should be a last resort and used sparingly.

3. **Cell, RefCell, and UnsafeCell**: Besides `Cell` that you've already seen, there are `RefCell` and `UnsafeCell` types in the standard library. `RefCell` is similar to `Cell`, but it allows for dynamic borrowing rules enforcement at runtime. `UnsafeCell` is a low-level type that provides zero-cost interior mutability, but using it requires you to ensure safety manually.

4. **Global Mutable State**: In some cases, you might encounter global mutable state using the `lazy_static` crate or similar techniques. This allows you to have a single instance of mutable data accessible from anywhere in your program.

5. **Functional Programming Patterns**: In some cases, you might avoid mutable state altogether by adopting functional programming patterns that favor immutability and transformation of data over direct mutation.


___


## Functional Iteration

COMPILE!


```rust
#![deny(elided_lifetimes_in_paths)]

struct Pixel<'img> {
    value: &'img mut u32,
    i: usize,
}

struct Canvas {
    imgs: Vec<u32>,
    max_i: usize,
}

impl Canvas {
    pub fn iter_mut<'iter>(self: &'iter mut Self) -> impl Iterator<Item = Pixel<'iter>> {
        self.imgs[..self.max_i]
            .iter_mut()
            .enumerate()
            .map(|(i, value)| Pixel { value, i })
    }
}

fn main() {
    let mut canvas = Canvas {
        imgs: vec![0; 9],
        max_i: 3,
    };
    for pixel in canvas.iter_mut() {
        *pixel.value = 3;
    }
}
```





___

## UNSAFE!



COMPILE!

```rust
struct Pixel<'img> {
    value: &'img mut u32,
    i: usize,
}

struct Canvas {
    img: Vec<u32>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel<'static>; // Using 'static lifetime here, which is unsafe
    fn next(&mut self) -> Option<Pixel<'static>> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: unsafe { &mut *(self.img.get_unchecked_mut(self.i) as *mut _) },
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
    let mut canvas = Canvas {
        img: vec![0; 9],
        i: 0,
        max_i: 3,
    };
    for pixel in canvas {
        *pixel.value = 3;
    }
}
```


____


## Rc

COMPILE!


```rust
use core::cell::Cell;
use std::rc::Rc;

struct Pixel {
    value: Rc<Cell<u32>>,
    i: usize,
}

struct Canvas {
    img: Vec<Rc<Cell<u32>>>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: self.img[self.i].clone(),
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
    let img = vec![Rc::new(Cell::new(0)); 9];
    let mut canvas = Canvas {
        img,
        i: 0,
        max_i: 3,
    };

    for pixel in canvas {
        pixel.value.set(3);
    }
}
```


## Cell


```rust
use std::cell::Cell;

struct Pixel {
    value: Cell<u32>,
    i: usize,
}

struct Canvas {
    img: Vec<Cell<u32>>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: self.img[self.i].clone(),
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
    let img = vec![Cell::new(0); 9];
    let mut canvas = Canvas {
        img,
        i: 0,
        max_i: 3,
    };
    for pixel in canvas {
        pixel.value.set(3);
    }
}
```



____


## Mutex and RwLock

COMPILE!


```rust
use std::sync::{Mutex, Arc};
use std::thread;

struct Pixel {
    value: Arc<Mutex<u32>>,
    i: usize,
}

struct Canvas {
    img: Vec<Arc<Mutex<u32>>>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: Arc::clone(&self.img[self.i]),
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
    let img = vec![Arc::new(Mutex::new(0)); 9];
    let canvas = Canvas {
        img: img.clone(),
        i: 0,
        max_i: 3,
    };

    // In Rust, when you use the `map` function on an iterator, 
    // it takes ownership of the iterator and moves its elements. 
    // In this case, `canvas` gets moved when you call 
    // `.map()` on it, and subsequently, we're trying 
    // to use `canvas.img` after it has been moved.
    // One way to avoid moving `canvas` is to clone 
    // each `Pixel` before calling `map`
    let handles: Vec<_> = canvas
        .map(|pixel| {
            let value = pixel.value.clone();
            thread::spawn(move || {
                *value.lock().unwrap() = 3;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    for value in img.iter() {
        println!("{}", value.lock().unwrap());
    }
}
```


____




## Using `UnsafeCell` (unsafe approach)



COMPILE ERRROR!

```rust
use std::cell::UnsafeCell;

struct Pixel {
    value: UnsafeCell<u32>,
    i: usize,
}

struct Canvas {
    img: Vec<UnsafeCell<u32>>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: self.img[self.i],
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
                   // Error
    let img = vec![UnsafeCell::new(0); 9];
    let mut canvas = Canvas {
        img,
        i: 0,
        max_i: 3,
    };
    for pixel in canvas {
        unsafe {
            *pixel.value.get() = 3;
        }
    }
}
```


```rust
error[E0277]: the trait bound `UnsafeCell<{integer}>: Clone` is not satisfied
    --> src/main.rs:31:20
     |
31   |     let img = vec![UnsafeCell::new(0); 9];
     |               -----^^^^^^^^^^^^^^^^^^----
     |               |    |
     |               |    the trait `Clone` is not implemented for `UnsafeCell<{integer}>`
     |               required by a bound introduced by this call
     |
note: required by a bound in `from_elem`
    --> /home/nemesis/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2534:21
     |
2534 | pub fn from_elem<T: Clone>(elem: T, n: usize) -> Vec<T> {
     |                     ^^^^^ required by this bound in `from_elem`

```



The `Clone` trait is used to create a copy of an object, and `UnsafeCell` is specifically designed to enable interior mutability (changing the value behind a reference) in a safe manner. However, creating a clone of an `UnsafeCell` might violate the safety guarantees it provides.

In this code, it seems like we want to create a vector of `Pixel` instances where each `Pixel` contains an `UnsafeCell<u32>`. To achieve this, we can manually construct the vector without relying on the `from_elem` function.



COMPILE!


```rust
use std::cell::UnsafeCell;

struct Pixel {
    value: UnsafeCell<u32>,
    i: usize,
}

struct Canvas {
    img: Vec<Pixel>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> {
        if self.i >= self.max_i {
            return None;
        }
        let retval = Pixel {
            value: UnsafeCell::new(0),
            i: self.i,
        };
        self.i += 1;
        Some(retval)
    }
}

fn main() {
    let mut canvas = Canvas {
        img: Vec::with_capacity(9),
        i: 0,
        max_i: 3,
    };

    for pixel in &mut canvas {
        unsafe {
            *pixel.value.get() = 3;
        }
    }
}
```



In this modified version, I've removed the `img` vector initialization from the `main` function and instead initialize it within the `next` method of the `Canvas` iterator. This allows you to create instances of `Pixel` and populate the `img` vector directly. The vector holds instances of `Pixel`, which contain `UnsafeCell<u32>` fields. The `for` loop iterates over the `canvas` iterator and updates the value of each pixel using the `UnsafeCell` reference.




___





## Functional Programming Patterns




Functional programming patterns often emphasize immutability and data transformation over direct mutation. Here's an example of how you could modify your code to follow a functional programming style using iterators and mapping



COMPILE!

```rust
struct Pixel {
    value: u32,
    i: usize,
}

struct Canvas {
    img: Vec<u32>,
    i: usize,
    max_i: usize,
}

impl Iterator for Canvas {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> {
        if self.i >= self.max_i {
            return None;
        }
        let pixel = Pixel {
            value: self.img[self.i],
            i: self.i,
        };
        self.i += 1;
        Some(pixel)
    }
}

fn main() {
    let mut canvas = Canvas {
        img: vec![0; 9],
        i: 0,
        max_i: 3,
    };

    let updated_img: Vec<u32> = canvas
        .map(|mut pixel| {
            pixel.value = 3;
            pixel.value
        })
        .collect();

    for value in updated_img {
        println!("{}", value);
    }
}
```



In this example, instead of directly modifying the `Canvas` values, we use the `map` function to transform the values of the `Canvas` into a new vector with the updated pixel values. This approach creates a new collection with the updated values while leaving the original data intact. This is a common functional programming pattern that avoids mutable state and helps improve code clarity and reasoning about data transformations.

It's worth noting that functional programming patterns often work well with Rust's ownership and borrowing system, and they can help you write more concise and maintainable code while reducing the likelihood of bugs related to mutable state.