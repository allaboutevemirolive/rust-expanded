Explain to me in detail about Polymorphism in the Rust programming language.



Polymorphism is a programming concept that allows a function or method to take on many forms, depending on the type of data it is given. In Rust, there are two primary ways to accomplish polymorphism: using generics and using traits. Generics allow you to write code that can work with any type, as long as that type meets certain requirements. Traits, on the other hand, allow you to define a set of methods that a type must implement in order to be used with a particular function or method. Both approaches have trade-offs to consider when it comes to performance as well as binary size[1].

Generics in Rust are called "parametric polymorphism" in type theory, which means that they are types or functions that have multiple forms over a given parameter. Rust's standard library provides a type, Option<T>, that's generic. For example, if we have a generic list of moveable things (like game characters or whatnot) holding different structs that implement the moveable trait and we loop through calling move on each, Rust will use static dispatch to determine which implementation of the moveable trait to use[10].

Traits are used to create ad-hoc polymorphism in Rust. A trait is a collection of methods that can be implemented by any type. When a type implements a trait, it can be used in any function or method that expects that trait. Traits can be used to define a set of methods that a type must implement in order to be used with a particular function or method. Rust's traits are similar to interfaces in other languages, but with some important differences. Rust's traits can have default implementations for some or all of their methods, and they can also have associated types and constants[4].

In Rust, polymorphic functions are fully type-checked when they are declared, not when they are used. This means you can never call a Rust function and get a type error within the function because you gave it the wrong type. Rust uses trait objects to implement dynamic dispatch, which allows you to write code that can work with any type that implements a particular trait, even if you don't know the exact type at compile time. Trait objects are represented as pointers to the type that implements the trait, along with a pointer to a table of function pointers that implement the trait's methods[8].

Rust has limited support for polymorphism. Rust allows limited, explicit coercion between numeric types using the "as" keyword. Rust has no function name overloading, and there is no inheritance in Rust. The nearest thing to a virtual method in Rust is a trait with an implemented function that an implementation overrides with its own. However, this override is at compile time[3].

Citations:
[1] https://oswalt.dev/2021/06/polymorphism-in-rust/
[2] https://youtube.com/watch?v=CHRNj5oubwc
[3] https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/polymorphism.html
[4] https://www.brandons.me/blog/polymorphism-in-rust
[5] https://www.reddit.com/r/rust/comments/12cvd9f/guidance_on_polymorphism_in_regards_to_trying_to/
[6] https://blog.logrocket.com/build-polymorphic-components-rust/
[7] https://www.thecodedmessage.com/posts/oop-2-polymorphism/
[8] https://stanford-cs242.github.io/f18/lectures/05-2-rust-traits.html
[9] https://users.rust-lang.org/t/polymorphism-in-rust-is-it-possible/81691
[10] https://www.reddit.com/r/rust/comments/20f28o/how_does_polymorphism_work_in_rust/
[11] https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/generics.html