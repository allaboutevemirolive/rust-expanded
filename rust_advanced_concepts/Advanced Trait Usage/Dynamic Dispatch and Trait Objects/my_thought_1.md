https://kerkour.com/rust-generics-trait-objects


The website above can be good starting point to understand what is `dynamic dispatch` and `dynamic dispatch`.


Before we deep dive into these concepts, we need to understand what is the process of `Monomorphization` and `Polymorphism` in Rust programming language.

## MEANING

`Monomorphization` is a compile-time process in Rust where polymorphic functions are replaced by many monomorphic functions for each unique instantiation. In other words, it is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

`Polymorphism` is a programming concept that allows a function or method to take on many forms, depending on the type of data it is given.


## Code example for `Monomorphization`


### 1




Before `Monomorphization`




COMPILE!

```rust
fn first<'a, A: ?Sized>(pair: (&'a A, &'a A)) -> &'a A {
    let (a, _) = pair;
    a
}

fn main() {
    let int_pair = (&1, &2);
    let str_pair = ("a", "b");

    let int_result = first(int_pair);
    let str_result = first(str_pair);

    println!("First element of int pair: {}", int_result);
    println!("First element of str pair: {}", str_result);
}
```

Output:

```
First element of int pair: 1
First element of str pair: a
```




After `Monomorphization`




```rust
fn first_int<'a>(pair: (&'a i32, &'a i32)) -> &'a i32 {
    let (a, _) = pair;
    a
}

fn first_str<'a>(pair: (&'a str, &'a str)) -> &'a str {
    let (a, _) = pair;
    a
}

fn main() {
    let int_pair = (&1, &2);
    let str_pair = ("a", "b");

    let int_result = first_int(int_pair);
    let str_result = first_str(str_pair);

    println!("First element of int pair: {}", int_result);
    println!("First element of str pair: {}", str_result);
}
```

Output:

```
First element of int pair: 1
First element of str pair: a
```


### 2




Before `Monomorphization`




```rust
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```




After `Monomorphization`




```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```



### 3



Before `Monomorphization`



```rust
trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
}

struct Risc {}

impl Processor for Risc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

struct Cisc {}

impl Processor for Cisc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x * y
    }
}

// monomorphization
fn process<P: Processor>(processor: &P, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}

pub fn main() {
    let processor1 = Cisc {};
    let processor2 = Risc {};

    process(&processor1, 1);
    process(&processor2, 2);
}
```




After `Monomorphization`




```rust
fn process_Risc(processor: &Risc, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}

fn process_Cisc(processor: &Cisc, x: i64) {
    let result = processor.compute(x, 42);
    println!("{}", result);
}
```


`Monomorphization` happens at compile time. The specialized versions of code are generated during the compilation process for each type used with generic functions or structs. This means that by the time your program is executed, the compiler has already generated separate code for each specific type. This leads to efficient and specialized machine code that doesn't involve any runtime overhead related to type resolution.

On the other hand, `polymorphism` using trait objects and dynamic dispatch happens at runtime. The method calls are resolved based on the actual type of the object when the code is executed. This dynamic dispatch allows you to write code that works with different types adhering to a common trait without having to generate separate code for each type. However, it introduces a small runtime overhead due to vtable lookups and method indirection.

In summary:

- **Monomorphization**: Specialized code is generated at compile time for each type used with generics. There is no runtime overhead related to type resolution.

- **Polymorphism**: Method calls are resolved at runtime based on the actual type of the object, allowing code to work with different types adhering to a shared trait. This introduces a small runtime overhead due to dynamic dispatch.

Both techniques have their strengths and trade-offs, and Rust provides developers with the flexibility to choose the most appropriate approach based on their specific use case and performance requirements.


___



Rust uses string manipulation techniques to generate specialized functions for different types in order to achieve monomorphization. However, it's not just about string manipulation; it's about code generation and specialization based on the specific types used with generic functions or structs.

When you write a generic function or struct in Rust, the compiler generates separate instances of that code for each concrete type that you use with it. This process is often referred to as "code specialization" or "code generation."

Here's a high-level overview of how this works:

1. **Type Specialization**: When you use a generic function or struct with a specific type, Rust's compiler analyzes your usage and identifies the actual type you're using.

2. **Code Generation**: The compiler generates a specialized version of the generic code specifically for that type. This specialized version eliminates the generic parameters and replaces them with the actual type.

3. **Compile-Time Optimizations**: Since the specialized code is tailored to the specific type, the compiler can perform optimizations and eliminate unnecessary checks, resulting in more efficient and specialized machine code.

4. **Reduced Code Duplication**: This approach avoids the need to generate redundant code for every possible combination of type parameters, reducing code bloat.

___


While in Polymorphism, Rust not generate specialized functions for different types but make the same code reusable?



Yes, that's correct. In the context of polymorphism, Rust uses trait objects and dynamic dispatch to achieve reusability of code without generating specialized functions for different types.

In polymorphism, you define a trait that specifies a set of methods that types can implement. Then, different types can implement that trait, and you can work with those types through the common trait interface. The key point here is that you're defining a common behavior that various types can adhere to.

When you use trait objects and dynamic dispatch, Rust doesn't generate specialized functions for each type. Instead, it generates a mechanism for runtime method resolution. The type-specific implementations of trait methods are determined at runtime based on the actual type of the object.

This has a few important implications:

1. **Code Reusability**: You can write code that works with multiple types that implement the same trait, without needing to manually generate specialized code for each type.

2. **Late Binding**: The specific method implementation is determined at runtime. This allows for late binding, where the behavior can be chosen at runtime based on the actual object type.

3. **Flexibility**: Trait objects and polymorphism enable you to work with types that you might not know at compile time, which is useful for creating libraries, frameworks, and APIs where users can define their own types that adhere to a common contract.

4. **Runtime Overhead**: Dynamic dispatch introduces a small runtime overhead due to vtable lookups and method indirection compared to monomorphization. However, this overhead is often negligible for many use cases.

Here's a recap of the key differences:

- **Monomorphization**: Generates specialized code for each type at compile time, leading to optimized code but potentially more code duplication.

- **Polymorphism**: Enables code reusability through trait objects and dynamic dispatch, allowing multiple types to share a common interface without the need for specialized code for each type.

In summary, monomorphization and polymorphism serve different purposes: monomorphization optimizes code for specific types, while polymorphism provides code reusability through shared traits and dynamic dispatch.


___



```
So monomorphization happens at compile time, where the code generated before runtime (code execution), 

while Polymorphism happens at runtime (code execution) without code generation?
```



Yes, you've got it right!

Monomorphization happens at compile time. The specialized versions of code are generated during the compilation process for each type used with generic functions or structs. This means that by the time your program is executed, the compiler has already generated separate code for each specific type. This leads to efficient and specialized machine code that doesn't involve any runtime overhead related to type resolution.

On the other hand, polymorphism using trait objects and dynamic dispatch happens at runtime. The method calls are resolved based on the actual type of the object when the code is executed. This dynamic dispatch allows you to write code that works with different types adhering to a common trait without having to generate separate code for each type. However, it introduces a small runtime overhead due to vtable lookups and method indirection.

In summary:

- **Monomorphization**: Specialized code is generated at compile time for each type used with generics. There is no runtime overhead related to type resolution.

- **Polymorphism**: Method calls are resolved at runtime based on the actual type of the object, allowing code to work with different types adhering to a shared trait. This introduces a small runtime overhead due to dynamic dispatch.

Both techniques have their strengths and trade-offs, and Rust provides developers with the flexibility to choose the most appropriate approach based on their specific use case and performance requirements.







