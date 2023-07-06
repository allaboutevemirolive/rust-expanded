# multiple lifetime parameter because input is nested lifetime parameter


```rust
struct Container<'a> {
    data: &'a str,
}

struct Processor<'a, 'b> {
    container: &'a Container<'b>,
}

fn process<'a, 'b>(processor: &'a Processor<'a, 'b>) -> &'b str
where
    'a: 'b,
{
    processor.container.data
}

fn main() {
    let data = "Hello, world!";
    let container = Container { data: &data };
    let processor = Processor { container: &container };
    let result = process(&processor);
    println!("Result: {}", result);
}
```

___

## Return type with shorter lifetime

I think it is worth mentioning that lifetime parameters in Rust are closely tied to the scopes in which they are defined. By assigning appropriate lifetime parameters and adding constraints, we ensure that references have valid lifetimes and avoid issues with dangling references or references outliving their intended scopes.

Each item or reference has its own lifetime, which determines how long it is valid and can be used. When we specify different lifetime parameters for different items, we are giving each of them a "ticket" that represents the duration for which the reference is valid.

Consider the following code structure:

```rust
{
    // Scope of item A
    {
        // Scope of item B
    }
}
```

Item B can only live within its scope, while item A can live in both the scope of item A and the scope of item B.



```rust
static ZERO: i32 = 0;

struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

// Returning lifetime 'a
fn get_x_or_zero_ref<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if *x > *y {
        return x
    } else {
        return &ZERO
    }
}

fn main() {
    let x = 1;
    let v;
    {
        let y = 2;
        let f = Foo { x: &x, y: &y };
        v = get_x_or_zero_ref(&f.x, &f.y);
    }
    println!("{}", *v);
}
```

In the code above, the `get_x_or_zero_ref` function has two input references with different lifetimes: `'a` for `x` and `'b` for `y`. By giving each of them a lifetime parameter, we are essentially giving them their own "tickets" that specify how long the reference is valid to avoid dangling references.

Now, when we specify the return type of the `get_x_or_zero_ref` function as `'a i32`, we are saying that the lifetime parameter for the return type should be the same as the lifetime parameter for the first input reference, which is `'a`. This ensures that the returned reference will not outlive the scope of `x`.

> But what if we want to specify the lifetime parameter for the return type as `'b i32` or `-> &'b i32` or that has a shorter lifetime instead?

In this case, we are trying to assign the lifetime parameter of `'b` to the return type, which corresponds to the scope of item B. However, there is a potential problem if we don't handle it correctly. The lifetime of item B will end within its scope, but the lifetime of item A, by default, has a greater scope than that of item B.

This is problematic because we specified the return type to have the lifetime `'b`, which corresponds to item B's scope. However, the lifetime of item A, represented by `'a`, outlives the needs of the return type. We are essentially giving the return type a ticket that is valid for a shorter duration than the actual lifetime of item A.

To resolve this issue, we need to add a constraint to the `'a` lifetime of item A, indicating that the tickets associated with `'a` are only valid within the scope of item B. By adding this constraint, we ensure that the returned reference does not outlive its intended scope and maintains the correct relationships between lifetimes.



So if we want to return an item that has a shorter lifetime, we need to determine if there is another input with a greater lifetime. Then we add a constraint on the input with a greater lifetime so its tickets are only valid within the shorter lifetime and don't outlive another.

Here is the code if we want to write the return type to have a shorter lifetime.

```rust
static ZERO: i32 = 0;

struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

// returning lifetime 'b
// Modified constrain with `where` clause
fn get_x_or_zero_ref<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32
where
    // Adding constrain so a will not outlive b
    'a: 'b,
{
    if *x > *y {
        return x;
    } else {
        return &ZERO;
    }
}

fn main() {
    let x = 1;
    let v;
    {
        let y = 2;
        let f = Foo { x: &x, y: &y };
        v = get_x_or_zero_ref(&f.x, &f.y);
        println!("{}", *v);
    }
}
```