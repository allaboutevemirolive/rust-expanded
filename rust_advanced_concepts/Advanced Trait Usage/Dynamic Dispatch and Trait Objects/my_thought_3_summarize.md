

`Static dispatch` uses `monomorphization`, which accepts any type (generic type) and then generates specific code for each specific type at `compile time`. 

While,

`Dynamic dispatch` use `polymorphism` that accepts objects that have a reference to a shared trait and executes at `runtime`.



___


- `Static dispatch` generates specialized code for each specific type at compile time using `monomorphization`, resulting in potentially better performance due to optimization.


```rust
trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
}
// monomorphization
                        // accept generic type
fn process<P: Processor>(processor: &P, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}
```

- `Dynamic dispatch` accepts objects that have references to a shared trait and decides which implementation to use at runtime, providing flexibility and runtime `polymorphism`.

```rust
trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
}
                            // accept object that has 
                            // the same trait
fn process_dynamic(processor: &dyn Processor, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}
```



1. **Static Dispatch (Monomorphization)**:
   - In static dispatch, the compiler generates specific code for each concrete type that a generic function or method is used with.
   - This process is known as monomorphization, where the compiler essentially creates separate instances of the function or method for each type.
   - The generated code is specialized and optimized for each specific type, resulting in potentially better performance.
   - Static dispatch occurs at compile time, and the exact implementation to be used is determined based on the type information available during compilation.

2. **Dynamic Dispatch**:
   - In dynamic dispatch, code is not specialized for specific types at compile time; instead, the appropriate implementation is determined and executed at runtime.
   - This is achieved through the use of trait objects, which are references or pointers to instances of types that implement a particular trait.
   - The specific implementation to be executed is decided based on the actual type of the trait object at runtime.
   - Dynamic dispatch allows for more flexibility and runtime polymorphism, as the decision about which implementation to use can change based on runtime conditions.

Both static and dynamic dispatch have their own use cases and trade-offs, and understanding when to use each approach is important for writing efficient and maintainable Rust code.