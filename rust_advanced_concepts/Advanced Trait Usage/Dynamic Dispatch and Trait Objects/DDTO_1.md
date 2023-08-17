Explain to me in detail about Dynamic Dispatch and Trait Objects in Rust programming language


## Dynamic Dispatch and Trait Objects in Rust

When code involves polymorphism, there needs to be a mechanism to determine which specific version is actually run. This is called ‘dispatch’. There are two major forms of dispatch: `static dispatch` and `dynamic dispatch`. While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called ‘trait objects’ [1][6].

### Static Dispatch

Static dispatch is the default dispatch mode that is used when the concrete type can be determined at compile time. In static dispatch, the compiler generates code that directly calls the method associated with the type. This is faster than dynamic dispatch because the method call is resolved at compile time, and there is no need for a runtime lookup[14].

### Dynamic Dispatch

Dynamic dispatch is used when the concrete type implementing the trait is not known at compile time. In dynamic dispatch, the compiler generates code that at runtime will figure out which method to call. This is slower than static dispatch because there is a runtime lookup, but it is more flexible because it allows for polymorphism[4][6].

### Trait Objects

Trait objects, like `&Foo` or `Box<Foo>`, are normal values that store a value of any type that implements the given trait, where the precise type can only be determined at runtime. When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call[1][4][5].

Trait objects are written as the keyword `dyn` followed by a set of trait bounds. A trait object is an opaque value of another type that implements a set of traits. The set of traits is made up of an object safe base trait plus any number of auto traits. Trait objects implement the base trait, its auto traits, and any supertraits of the base trait[7].

Trait objects are more like objects in other languages in the sense that they combine data and behavior. But trait objects differ from traditional objects in that they do not have a fixed type. Instead, they can store values of any type that implements the given trait[2].

The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway. Rust won’t compile our code if the values don’t implement the traits that the trait objects need[4].

Trait objects are useful when we want our library user to be able to extend the set of types that are valid in a particular situation. They allow for dynamic polymorphism and heterogeneous uses of types[10].

In conclusion, Rust supports both static and dynamic dispatch. While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called trait objects. Trait objects are useful when we want to allow for dynamic polymorphism and heterogeneous uses of types.

Citations:
[1] https://doc.rust-lang.org/1.8.0/book/trait-objects.html
[2] https://doc.rust-lang.org/book/ch17-02-trait-objects.html
[3] https://en.wikipedia.org/wiki/Rust_(programming_language)
[4] https://doc.rust-lang.org/book/ch17-02-trait-objects.html?highlight=dynamic+dispatch
[5] https://joshleeb.com/posts/rust-traits-and-trait-objects/
[6] https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/static-and-dynamic-dispatch.html
[7] https://doc.rust-lang.org/reference/types/trait-object.html
[8] https://www.rust-lang.org
[9] https://youtube.com/watch?v=tM2r9HD4ivQ
[10] https://brson.github.io/rust-anthology/1/all-about-trait-objects.html
[11] https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
[12] https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/trait-objects.html
[13] https://www.geeksforgeeks.org/introduction-to-rust-programming-language/
[14] https://www.eventhelix.com/rust/rust-to-assembly-static-vs-dynamic-dispatch/
[15] https://stackoverflow.com/questions/27567849/what-makes-something-a-trait-object
[16] https://youtube.com/watch?v=5C_HPTJg5ek
[17] https://www.reddit.com/r/rust/comments/rggekc/when_is_it_better_to_use_static_vs_dynamic/
[18] https://practice.rs/generics-traits/trait-object.html
[19] https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/
[20] https://www.linkedin.com/pulse/dynamic-static-dispatch-rust-amit-nadiger
[21] https://youtube.com/watch?v=ReBmm0eJg6g
[22] https://tourofrust.com/81_en.html
[23] https://www.linkedin.com/advice/0/how-do-you-choose-between-trait-objects-generics
[24] https://gist.github.com/greister/37289c6eb3629d4fefa7dd0acf6de378
[25] https://google.github.io/comprehensive-rust/traits/trait-objects.html