Static dispatch and dynamic dispatch are two major forms of dispatch in Rust programming language. Static dispatch is used when the concrete type can be determined at compile time, while dynamic dispatch is used when the concrete type implementing the trait is not known at compile time. Rust generates code that directly calls the method associated with the type in static dispatch, while in dynamic dispatch, the compiler generates code that at runtime will figure out which method to call. Rust supports dynamic dispatch through a mechanism called trait objects. Trait objects are normal values that store a value of any type that implements the given trait, where the precise type can only be determined at runtime. When we use trait objects, Rust must use dynamic dispatch. Trait objects are useful when we want to allow for dynamic polymorphism and heterogeneous uses of types. 

Here are some Rust code examples that implement static dispatch and dynamic dispatch:

### Static Dispatch

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Shape for Rectangle<T>
where
    T: std::ops::Mul<Output = T> + Copy + Into<f64>,
{
    fn area(&self) -> f64 {
        let w: f64 = self.width.into();
        let h: f64 = self.height.into();
        w * h
    }
}

fn area_pair_static<T: Shape>(a: T, b: T) -> (f64, f64) {
    (a.area(), b.area())
}

fn static_dispatch_pair(a: Rectangle<f64>, b: Rectangle<f64>) -> (f64, f64) {
    area_pair_static(a, b)
}
```

In this example, we define a trait `Shape` and a struct `Rectangle` that implements the `Shape` trait. We then define a function `area_pair_static` that takes two arguments of any type that implements the `Shape` trait and returns a tuple of their areas. Finally, we define a function `static_dispatch_pair` that takes two `Rectangle<f64>` arguments and calls `area_pair_static` with them. Since the concrete type of the arguments is known at compile time, Rust uses static dispatch to call the appropriate method.

### Dynamic Dispatch

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button with label: {}", self.label);
    }
}

struct TextField {
    value: String,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing text field with value: {}", self.value);
    }
}

fn draw_all(drawables: &[&dyn Draw]) {
    for drawable in drawables {
        drawable.draw();
    }
}

fn dynamic_dispatch() {
    let button = Button {
        label: String::from("Click me!"),
    };
    let text_field = TextField {
        value: String::from("Hello, world!"),
    };
    let drawables: &[&dyn Draw] = &[&button, &text_field];
    draw_all(drawables);
}
```

In this example, we define a trait `Draw` and two structs `Button` and `TextField` that implement the `Draw` trait. We then define a function `draw_all` that takes a slice of trait objects that implement the `Draw` trait and calls their `draw` method. Finally, we define a function `dynamic_dispatch` that creates two instances of `Button` and `TextField`, puts them in a slice of trait objects, and calls `draw_all` with the slice. Since the concrete type of the objects is not known at compile time, Rust uses dynamic dispatch to call the appropriate method at runtime.

Citations:
[1] https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html
[2] https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
[3] https://www.linkedin.com/pulse/dynamic-static-dispatch-rust-amit-nadiger
[4] https://doc.rust-lang.org/book/ch17-02-trait-objects.html?highlight=dynamic+dispatch
[5] https://www.eventhelix.com/rust/rust-to-assembly-static-vs-dynamic-dispatch/
[6] https://kerkour.com/rust-generics-trait-objects
[7] https://www.reddit.com/r/rust/comments/ta2cei/why_exactly_cant_you_always_do_static_dispatch_in/
[8] https://gist.github.com/greister/37289c6eb3629d4fefa7dd0acf6de378
[9] https://riptutorial.com/rust/example/4656/static-and-dynamic-dispatch
[10] https://internals.rust-lang.org/t/add-trait-objects-with-static-dispatch/16069
[11] https://jmmv.dev/2023/08/rust-static-dispatch-failed-experiment.html