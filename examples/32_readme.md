https://stackoverflow.com/questions/30422177/how-do-i-write-an-iterator-that-returns-references-to-itself







COMPILE ERROR!


```rust
use std::iter::Iterator;

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T> Iterator for PermutationIterator<T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Vec<&T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                result.push(self.vs[i].get(index).unwrap());
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    // TODO: Can improve design
    loop {
        match i.next() {
            Some(v) => {
                println!("{:?}", v);
            }
            None => {
                break;
            }
        }
    }
}
```

```rust
   Compiling test_code v0.1.0 (/home/nemesis/Documents/Github/my_repo/rust-expanded/test_code)
error[E0261]: use of undeclared lifetime name `'a`
  --> src/main.rs:23:22
   |
23 |     type Item = Vec<&'a T>;
   |                      ^^ undeclared lifetime
   |
help: consider introducing lifetime `'a` here
   |
23 |     type Item<'a> = Vec<&'a T>;
   |              ++++
help: consider introducing lifetime `'a` here
   |
22 | impl<'a, T> Iterator for PermutationIterator<T> {
   |      +++

error[E0311]: the parameter type `T` may not live long enough
  --> src/main.rs:23:17
   |
23 |     type Item = Vec<&'a T>;
   |                 ^^^^^^^^^^ ...so that the reference type `&T` does not outlive the data it points at

Some errors have detailed explanations: E0261, E0311.
For more information about an error, try `rustc --explain E0261`.
error: could not compile `test_code` (bin "test_code") due to 2 previous errors
```


Issues in the code:

1. **Undeclared Lifetime `'a`**: The undeclared lifetime `'a` in `type Item = Vec<&'a T>;` causes an error because there's no lifetime `'a` defined in the struct or function signature.

2. **Lifetime `'a` Usage**: In the `Iterator` implementation, you are trying to use a lifetime `'a` without having any lifetime parameter `'a` defined for the struct or function.

3. **Lifetime Mismatch**: The use of lifetimes in the `Iterator` implementation leads to a conflict with the lifetime `'a`, which is not even defined in the current scope. The lifetime `'a` is not connected to any lifetime of references in the struct or the function parameters.

4. **Reference Lifetime**: The reference type `&'a T` in `type Item = Vec<&'a T>;` suggests that you want to return references to elements in the vector, but the lifetime `'a` is not properly connected to the actual lifetimes of the references.








___



COMPILE!


```rust
use std::iter::Iterator;

struct PermutationIterator<'a, T: 'a> {
    vs: Vec<&'a [T]>,
    is: Vec<usize>,
}

impl<'a, T> PermutationIterator<'a, T> {
    fn new() -> PermutationIterator<'a, T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: &'a [T]) {
        self.vs.push(v);
        self.is.push(0);
    }
}

// change
impl<'a, T> Iterator for PermutationIterator<'a, T> {
    // change
    type Item = Vec<&'a T>;
    // change
    fn next(&mut self) -> Option<Vec<&'a T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                result.push(self.vs[i].get(index).unwrap());
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<i32> = (1..3).collect();
    let v2: Vec<i32> = (3..5).collect();
    let v3: Vec<i32> = (1..6).collect();

    let mut i = PermutationIterator::new();
    // change
    i.add(&v1);
    i.add(&v2);
    i.add(&v3);

    loop {
        match i.next() {
            Some(v) => {
                println!("{:?}", v);
            }
            None => {
                break;
            }
        }
    }
}
```



___




## Functional Approach_1


COMPILE!


```rust
struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                                             // clone
                result.push(self.vs[i][index].clone());
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    while let Some(v) = i.next() {
        println!("{:?}", v);
    }
}
```


___



## Functional Approach_2


COMPILE 


```rust
use std::iter::Iterator; 

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T: Clone + Send> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let result: Vec<T> = self
                .vs
                .iter()
                .enumerate()
                .map(|(i, v)| v[self.is[i]].clone())
                .collect();

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    for v in i {
        println!("{:?}", v);
    }
}
```






___



## Unsafe approach


COMPILE

```rust
use std::iter::Iterator;

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T> Iterator for PermutationIterator<T> {
    type Item = Vec<*const T>; // Use *const T instead of &T

    fn next(&mut self) -> Option<Vec<*const T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None;
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                result.push(&self.vs[i][index] as *const T); // Convert to *const T
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    loop {
        match i.next() {
            Some(v) => {
                let v_refs: Vec<&i32> = v.iter().map(|&ptr| unsafe { &*ptr }).collect();
                println!("{:?}", v_refs);
            }
            None => {
                break;
            }
        }
    }
}
```


In this code, we change the `type Item` to `Vec<*const T>`, which allows us to store raw pointers to the elements instead of references. We then use `as *const T` to convert the references into raw pointers. In the `main` function, we use `unsafe { &*ptr }` to convert the raw pointers back to references when printing. This approach avoids the lifetime issues associated with the original code. However, using `unsafe` requires careful handling to ensure memory safety.




___


## Mutex and RwLock


COMPILE

```rust
use std::iter::Iterator;
use std::sync::{Arc, RwLock};

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                result.push(self.vs[i][index].clone());
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    let shared_iterator = Arc::new(RwLock::new(i));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let shared_iterator = shared_iterator.clone();
            std::thread::spawn(move || loop {
                let mut iterator = shared_iterator.write().unwrap();
                match iterator.next() {
                    Some(v) => {
                        println!("{:?}", v);
                    }
                    None => {
                        break;
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```


Mutex and RwLock can be used to address the lifetime and ownership issues. In this example, we're using an `Arc<RwLock<PermutationIterator<T>>>` to ensure safe concurrent access to the iterator. Multiple threads can now read from and modify the iterator without conflicting with each other.


Mutex (short for "mutual exclusion") and RwLock (short for "read-write lock") are synchronization primitives in Rust that are used to manage concurrent access to shared data in a multi-threaded environment. Their main purpose is to prevent data races, which occur when multiple threads access and modify shared data concurrently, leading to unpredictable and erroneous behavior.

Here's a brief explanation of each:

1. **Mutex:**
   Mutex is used when you want to ensure that only one thread can access a piece of data at a time. It enforces mutual exclusion by allowing one thread to lock the mutex and gain access to the protected data, while other threads are blocked until the mutex is unlocked. Mutexes are suitable when you have data that is being updated or modified by multiple threads and you want to ensure that only one thread is modifying it at any given time.

2. **RwLock:**
   RwLock allows multiple threads to read the shared data simultaneously, but only one thread can hold a write lock and modify the data. This is useful in scenarios where data is frequently read but infrequently modified. When a thread holds a read lock, other threads can also acquire read locks, but no thread can acquire a write lock until all read locks are released. This approach maximizes concurrency for reading while still ensuring that writes are protected.


The problem you're facing in the initial code is primarily related to lifetimes and references, as you correctly pointed out. Mutex and RwLock can help address the lifetime and ownership issues in a concurrent context. 

The key idea here is that by using these synchronization primitives, you can safely share and access the iterator across multiple threads.



___


## RefCell


COMPILE:


```rust
use std::cell::RefCell;
use std::iter::Iterator;

struct PermutationIterator<T> {
    vs: Vec<RefCell<Vec<T>>>,
    is: Vec<RefCell<usize>>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(RefCell::new(v));
        self.is.push(RefCell::new(0));
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if *self.is[i].borrow() >= self.vs[i].borrow().len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    *self.is[i].borrow_mut() = 0;
                    *self.is[i - 1].borrow_mut() += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = *self.is[i].borrow();
                result.push(self.vs[i].borrow()[index].clone());
            }

            *self.is.last().unwrap().borrow_mut() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    loop {
        match i.next() {
            Some(v) => {
                println!("{:?}", v);
            }
            None => {
                break;
            }
        }
    }
}
```




___


## UnsafeCell


COMPILE:


```rust
use std::cell::UnsafeCell;
use std::iter::Iterator;

struct PermutationIterator<T> {
    vs: Vec<UnsafeCell<Vec<T>>>,
    is: Vec<UnsafeCell<usize>>,
}

unsafe impl<T> Sync for PermutationIterator<T> {} // Unsafe synchronization due to UnsafeCell

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(UnsafeCell::new(v));
        self.is.push(UnsafeCell::new(0));
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                let is_i = unsafe { &*self.is[i].get() };
                if *is_i >= unsafe { &*self.vs[i].get() }.len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    let is_i = unsafe { &mut *self.is[i].get() };
                    *is_i = 0;
                    let is_prev = unsafe { &mut *self.is[i - 1].get() };
                    *is_prev += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = unsafe { *self.is[i].get() };
                result.push(unsafe { &*self.vs[i].get() }[index].clone());
            }

            let last_is = unsafe { &mut *self.is.last().unwrap().get() };
            *last_is += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    loop {
        match i.next() {
            Some(v) => {
                println!("{:?}", v);
            }
            None => {
                break;
            }
        }
    }
}
```

In this version, `UnsafeCell` is used to achieve interior mutability. This code compiles and runs, but it comes with several caveats:

1. The `Sync` trait implementation with `unsafe impl` means that you're indicating that the `PermutationIterator` type can be safely shared between threads even though it contains interior mutability. However, this isn't necessarily true, and using `unsafe impl Sync` with `UnsafeCell` can introduce data races and undefined behavior. It's generally not recommended to use `UnsafeCell` in a multithreaded context without careful consideration.

2. This code uses `unsafe` blocks to dereference the `UnsafeCell` pointers, indicating that you're taking responsibility for ensuring that the operations are safe.

3. Using `UnsafeCell` requires a deep understanding of Rust's memory safety model and a clear reason for bypassing it. It's not a recommended approach in most cases due to the potential risks it introduces.

4. If your goal is to share mutable data between threads, using `Arc<Mutex<Vec<T>>>` or `Arc<RwLock<Vec<T>>>` (from the `std::sync` module) might be more appropriate as they provide safer ways to achieve shared mutable access while handling synchronization correctly.

In summary, while it's possible to use `UnsafeCell` to achieve interior mutability, it's highly recommended to consider safer alternatives like `RefCell` or synchronization primitives (`Mutex`, `RwLock`) from the standard library, especially when dealing with multithreaded scenarios. Using `UnsafeCell` should be a well-justified decision with a deep understanding of its implications.



```
Do an analysis between Cell, RefCell, and UnsafeCell. Which is appropriate to solve the initial problem code and which is not? Give me the reason too
```


Certainly, I'll provide an analysis of `Cell`, `RefCell`, and `UnsafeCell` in the context of solving the initial problem code.

1. **`Cell`:**
   `Cell` provides a simple way to achieve interior mutability for single-threaded scenarios. It allows mutation of its content while ensuring Rust's safety guarantees. However, `Cell` can only be used for types that implement `Copy`, as it works by directly replacing the old value with the new value without any memory management.

   In your problem code, where you're dealing with vectors that do not implement `Copy` and need more dynamic behavior, `Cell` is not appropriate. It cannot be used to handle mutable vectors and their references simultaneously.

2. **`RefCell`:**
   `RefCell` offers interior mutability for scenarios where multiple references to the same data need to coexist with mutable access. It enforces Rust's borrowing rules at runtime using runtime checks. If these rules are violated, a panic occurs.

   In your problem code, `RefCell` is a suitable choice. It allows you to have multiple references to your vectors while still enabling mutable access when needed. You can modify the content of vectors within a `RefCell` and borrow them immutably as long as there are no active mutable references.

   However, be cautious when using `RefCell`. Runtime checks introduce a performance overhead, and improper use of `RefCell` can lead to panics. It's best suited for scenarios where you need dynamic mutability and can't achieve it using Rust's standard borrowing rules.

3. **`UnsafeCell`:**
   `UnsafeCell` provides the highest level of flexibility for interior mutability. It completely bypasses Rust's safety guarantees and hands the responsibility of ensuring safety to the programmer. You must use `UnsafeCell` carefully and understand the possible consequences and risks.

   In your problem code, you could potentially use `UnsafeCell` to manage the interior mutability of the vectors. However, this approach is not recommended because there are safer alternatives available like `RefCell`, which still enforce some level of safety at runtime.

**Recommendation:**
In your specific problem, where you want to mutate vectors while having references to them, `RefCell` is the most appropriate choice. It provides a good balance between dynamic mutability and safety. It ensures that Rust's borrowing rules are respected at runtime, preventing many common programming mistakes.

While `UnsafeCell` might technically work, it's considered a last resort for situations where other options aren't feasible due to the risks associated with bypassing Rust's safety mechanisms. `Cell` is not suitable for your use case because it can't handle non-`Copy` types like vectors.

In summary, use `RefCell` for interior mutability when you need to modify data through shared references while maintaining Rust's safety guarantees. If `RefCell` doesn't meet your requirements, carefully evaluate whether `UnsafeCell` is truly necessary and consider its implications on code safety and maintainability.





____


## Arc<Mutex<Vec<T>>>


COMPILE!


```rust
use std::sync::{Arc, Mutex};
use std::iter::Iterator;

struct PermutationIterator<T> {
    vs: Vec<Arc<Mutex<Vec<T>>>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(Arc::new(Mutex::new(v)));
        self.is.push(0);
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                let is_i = self.is[i];
                if is_i >= self.vs[i].lock().unwrap().len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                let value = self.vs[i].lock().unwrap()[index].clone();
                result.push(value);
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    // loop {
    //     match i.next() {
    //         Some(v) => {
    //             println!("{:?}", v);
    //         }
    //         None => {
    //             break;
    //         }
    //     }
    // }

    while let Some(v) = i.next() {
        println!("{:?}", v)
    }
}
```




____


## Arc<RwLock<Vec<T>>>


```rust
use std::iter::Iterator;
use std::sync::{Arc, RwLock};

struct PermutationIterator<T> {
    vs: Vec<Arc<RwLock<Vec<T>>>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(Arc::new(RwLock::new(v)));
        self.is.push(0);
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                let is_i = self.is[i];
                if is_i >= self.vs[i].read().unwrap().len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                let value = self.vs[i].read().unwrap()[index].clone();
                result.push(value);
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    loop {
        match i.next() {
            Some(v) => {
                println!("{:?}", v);
            }
            None => {
                break;
            }
        }
    }
}
```


___


## Channels and Message Passing


COMPILE!


```rust
use std::iter::Iterator;
use std::sync::mpsc;

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let mut result = vec![];

            for i in 0..self.vs.len() {
                let index = self.is[i];
                result.push(self.vs[i][index].clone());
            }

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let (tx, rx) = mpsc::channel();

    let mut i = PermutationIterator::new();
    i.add(v1.clone());
    i.add(v2.clone());
    i.add(v3.clone());

    std::thread::spawn(move || {
        while let Some(v) = i.next() {
            tx.send(v).unwrap();
        }
    });

    while let Ok(v) = rx.recv() {
        println!("{:?}", v);
    }
}
```



____


## Data Parallelism




COMPILE!


```rust
use rayon::prelude::*;
use std::iter::Iterator; 

struct PermutationIterator<T> {
    vs: Vec<Vec<T>>,
    is: Vec<usize>,
}

impl<T> PermutationIterator<T> {
    fn new() -> PermutationIterator<T> {
        PermutationIterator {
            vs: vec![],
            is: vec![],
        }
    }

    fn add(&mut self, v: Vec<T>) {
        self.vs.push(v);
        self.is.push(0);
    }
}

impl<T: Clone + Send> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        'outer: loop {
            for i in 0..self.vs.len() {
                if self.is[i] >= self.vs[i].len() {
                    if i == 0 {
                        return None; // we are done
                    }
                    self.is[i] = 0;
                    self.is[i - 1] += 1;
                    continue 'outer;
                }
            }

            let result: Vec<T> = self
                .vs
                .iter()
                .enumerate()
                .map(|(i, v)| v[self.is[i]].clone())
                .collect();

            *self.is.last_mut().unwrap() += 1;

            return Some(result);
        }
    }
}

fn main() {
    let v1: Vec<_> = (1..3).collect();
    let v2: Vec<_> = (3..5).collect();
    let v3: Vec<_> = (1..6).collect();

    let mut i = PermutationIterator::new();
    i.add(v1);
    i.add(v2);
    i.add(v3);

    for v in i {
        println!("{:?}", v);
    }
}
```