# Lifetime gets smaller after extracting method


code 1:

```rust
pub struct Obj {
    value: usize,
}

pub struct Container<'a> {
    content: &'a Obj,
}

pub struct Props<'a> {
    att: Container<'a>,
}

impl <'a> Props<'a> {
    pub fn value(&self) -> usize {
        self.att.content.value
    }
}

pub fn test<'a>(properties: Props<'a>) -> impl (Fn() -> usize) + 'a {
    || properties.att.content.value // works
    //|| properties.value() // works not
}
```


code 2:

```rust
pub struct Obj {
    value: usize,
}

pub struct Container<'a> {
    content: &'a Obj,
}

pub struct Props<'a> {
    att: Container<'a>,
}

impl <'a> Props<'a> {
    pub fn value(&self) -> usize {
        self.att.content.value
    }
}

pub fn test<'a>(properties: Props<'a>) -> impl (Fn() -> usize) + 'a {
    || properties.att.content.value // works
    // || properties.value() // works not
}

pub fn calltest<'a>(content:&'a Obj) -> impl (Fn() -> usize) + 'a {
    let att = Container {content};
    test(Props {att})
}
```

In the given code, the line `-> impl (Fn() -> usize) + 'a` is specifying the return type of the functions `test()` and `calltest()`. Let's break it down:

1. `impl (Fn() -> usize)`: This indicates that the functions will return a type that implements the `Fn()` trait, which represents a function that takes no arguments and returns a value of type `usize`.

2. `+ 'a`: This part is specifying the lifetime `'a` for the returned type. It means that the returned closure will have a lifetime tied to the lifetime of the input parameter `properties` in the `test()` function and the `content` parameter in the `calltest()` function.

By using `impl (Fn() -> usize) + 'a` as the return type, the functions `test()` and `calltest()` can return closures that capture and return the value of `usize`. The lifetime `'a` ensures that the closure references are valid for as long as the input parameters they capture are valid.

Regarding the commented line `|| properties.value()`, it doesn't work because the `value()` method is defined on the `Props` struct, which requires the `&self` parameter. In the closure context, `self` is not available, so you need to access the value through `properties.att.content.value` instead.