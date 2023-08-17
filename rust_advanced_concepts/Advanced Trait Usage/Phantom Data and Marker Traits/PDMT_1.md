Phantom Data and Marker Traits are two concepts in Rust programming language that are often used together. PhantomData is a zero-sized type that is used to mark things that "act like" they own a certain type T. Adding a PhantomData<T> field to a struct or an enum tells the compiler that the struct or enum acts as though it stores a value of type T, even though it doesn't really. This information is used when computing certain safety properties[1][5]. 

PhantomData is often used with marker traits to add additional behavior or properties to a type without changing its representation. Marker traits are traits that don't have any items. They are used to mark types as having some property. They usually don't have any associated types and methods and are used to tell the Rust compiler about functionality a type must provide. Rust has a handful of "markers" that classify types such as Send, Sync, Copy, Sized. These markers are just traits with empty bodies, which can then be used in both generics and trait objects. Markers can be defined in libraries, and they automatically provide [derive]-style implementations[2][9][13].

PhantomData is useful when you need to associate some additional type information into a struct or enum without actually using that type in any of the struct or enum's fields. PhantomData can be used to represent ownership or borrowing relationships between types, which can help the Rust borrow checker understand how the types are related. PhantomData can also be used to enforce certain invariants or constraints at the type level[3][5].

Here are some examples of how PhantomData and marker traits are used in Rust:
- A struct that has an unused lifetime parameter, typically as part of some unsafe code, can use PhantomData to tell the Rust compiler that the lifetime parameter is still valid[1][5].
- PhantomData can be used to simulate a field of a certain type for the purpose of static analysis. For example, PhantomData<&'a T> is used to indicate that a structure is tied to the lifetime 'a[5].
- PhantomData can be used to indicate ownership of data of a certain type T. This in turn implies that when the struct is dropped, it may drop one or more instances of the type T. This has bearing on the Rust compiler's drop check analysis[5][14].
- PhantomData can be used with marker traits to add additional behavior or properties to a type without changing its representation[2][9][13].

In summary, Phantom Data and Marker Traits are two concepts in Rust programming language that are often used together. PhantomData is a zero-sized type that is used to mark things that "act like" they own a certain type T. Marker traits are traits that don't have any items and are used to mark types as having some property. PhantomData is often used with marker traits to add additional behavior or properties to a type without changing its representation. PhantomData can be used to represent ownership or borrowing relationships between types, which can help the Rust borrow checker understand how the types are related. PhantomData can also be used to enforce certain invariants or constraints at the type level.

Citations:
[1] https://doc.rust-lang.org/stable/std/marker/struct.PhantomData.html
[2] https://users.rust-lang.org/t/understanding-the-marker-traits/75625
[3] https://codedamn.com/news/uncategorized/rusts-type-system-exploring-type-inference-phantom-data-associated-types
[4] https://serokell.io/blog/rust-traits
[5] https://doc.rust-lang.org/nomicon/phantom-data.html
[6] https://doc.rust-lang.org/std/marker/index.html
[7] https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/traits.html
[8] https://medium.com/@0xor0ne/rust-notes-phantomdata-505757bf56a7
[9] https://blog.rust-lang.org/2015/05/11/traits.html
[10] https://www.reddit.com/r/rust/comments/x58yhd/i_just_discovered_what_phantomdata_is_for/
[11] https://stackoverflow.com/questions/76445707/what-is-a-marker-trait-in-rust
[12] https://www.geeksforgeeks.org/rust-phantom-type-parameter/
[13] https://blog.logrocket.com/rust-traits-a-deep-dive/
[14] https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/marker/struct.PhantomData.html
[15] https://alexeden.github.io/learning-rust/programming_rust/13_utility_traits.html
[16] https://stackoverflow.com/questions/61308487/how-exactly-does-phantomdata-work-in-rust
[17] https://rust-lang.github.io/rfcs/1268-allow-overlapping-impls-on-marker-traits.html
[18] https://github.com/rust-lang/rust/issues/22914
[19] https://www.reddit.com/r/rust/comments/ckbgn4/how_to_implement_marker_trait_for_every_type/