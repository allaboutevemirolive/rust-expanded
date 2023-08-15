
Reference:
https://www.judy.co.uk/blog/rust-traits-and-references/


## Without blanket implementation

```rust

// This is what happen when we dont use blanket implementation
// We need to define Implementation 1 and Implementation 2, 2 times
// even the code is same.


/// A trait for things that can speak.
trait Speaker {
    fn speak(&self);
}

fn speak_to(s: impl Speaker) {
    s.speak();
}

// =====================================

// Implementation 1:

/// BasicSpeaker is an empty struct that exists only to implement Speaker.
struct BasicSpeaker;

/// BasicSpeakers must be able to speak.
impl Speaker for BasicSpeaker {
    fn speak(&self) {
        println!("Hello!");
    }
}

/// BasicSpeaker _references_ must also be able to speak.
impl Speaker for &BasicSpeaker {
    // Take a reference to reference speaker => &(&self)
    fn speak(&self) {
        return (**self).speak();
    }
}

// =====================================

// Implementation 2:
// Duplicate!

struct NamedSpeaker;

impl Speaker for NamedSpeaker {
    fn speak(&self) {
        println!("Hello Speaker!");
    }
}

impl Speaker for &NamedSpeaker {
    fn speak(&self) {
        return (**self).speak();
    }
}

// =====================================

fn main() {
    // Create a BasicSpeaker struct:
    let speaker: BasicSpeaker = BasicSpeaker;
    
    // I've commented out this function call, because it drops `speaker`:
    // We can solve this by implementing #[derive(Clone)] to struct BasicSpeaker
    // speak_to(speaker.clone());

    // Take a reference to the BasicSpeaker:
    let speaker_ref: &BasicSpeaker = &speaker;
    // Call the speak method defined on the BasicSpeaker, via the reference:
    speak_to(speaker_ref);


    let namedspeaker: NamedSpeaker = NamedSpeaker;
    let namedspeaker_ref: &NamedSpeaker = &namedspeaker;
    speak_to(namedspeaker_ref);

}
```




We cannot directly implement a blanket implementation for specific types like `BasicSpeaker` and `NamedSpeaker` in the same way you did for references, because blanket implementations apply to a range of types, not specific ones.


## With blanket implementation!

```rust
trait Speaker {
    fn speak(&self);
}

fn speak_to(s: impl Speaker) {
    s.speak();
}

// Blanket implementation for any type T that implements the Speaker trait
impl<T: Speaker> Speaker for &T {
    fn speak(&self) {
        (*self).speak();
    }
}

struct BasicSpeaker;
impl Speaker for BasicSpeaker {
    fn speak(&self) {
        println!("Hello!");
    }
}

struct NamedSpeaker;
impl Speaker for NamedSpeaker {
    fn speak(&self) {
        println!("Hello Speaker!");
    }
}

fn main() {
    let speaker: BasicSpeaker = BasicSpeaker;
    let speaker_ref: &BasicSpeaker = &speaker;
    speak_to(speaker_ref);

    let namedspeaker: NamedSpeaker = NamedSpeaker;
    let namedspeaker_ref: &NamedSpeaker = &namedspeaker;
    speak_to(namedspeaker_ref);
}
```


```
Why do we need blanket implementation? How does blanket implementation solve problem in rust code?
```


Blanket implementations in Rust provide a way to define a default implementation for a trait that applies to all types that satisfy certain conditions. This can be extremely useful for reducing code duplication and making your code more generic and flexible. Let's discuss how blanket implementations can solve the problem in your provided Rust code.

In your original code, you have multiple types (`BasicSpeaker` and `NamedSpeaker`) that implement the `Speaker` trait, and you've provided separate implementations for each type and their references. This can lead to code duplication and increased maintenance effort if you have many types that implement the same trait.

Here's a summary of the problem you're facing in your original code:

1. You have to define the same implementation for both the type and its reference (e.g., `impl Speaker for BasicSpeaker` and `impl Speaker for &BasicSpeaker`).
2. If you add more types that implement the `Speaker` trait, you'll need to define these implementations for each new type and its reference, leading to more duplicated code.

Now, let's see how blanket implementations can solve this problem:

1. **Reduced Code Duplication:** Instead of defining separate implementations for each type and its reference, you can define a single blanket implementation that covers all types that implement the `Speaker` trait. This reduces the amount of code you need to write and maintain.

2. **Flexibility:** With a blanket implementation, you can easily add new types that implement the `Speaker` trait without having to modify the trait's implementation code. The new types and their references automatically benefit from the blanket implementation.

3. **Easier Maintenance:** When you want to change the behavior of all types implementing the `Speaker` trait, you only need to modify the blanket implementation once, instead of finding and updating multiple individual implementations.

In the updated code example I provided earlier, the blanket implementation for `impl<T: Speaker> Speaker for &T` allows you to simplify the code by removing the need to write separate implementations for references to each type implementing the `Speaker` trait. This means that as long as a type implements the `Speaker` trait, you can directly call `speak_to` with a reference to that type, and it will work without needing any additional code.

In summary, blanket implementations in Rust help solve the problem of code duplication and make your code more maintainable, extensible, and efficient by providing a default implementation for a trait that applies to a range of types meeting certain criteria.