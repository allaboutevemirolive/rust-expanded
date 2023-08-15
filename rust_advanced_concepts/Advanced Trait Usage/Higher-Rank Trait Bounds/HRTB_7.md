


COMPILE ERROR

```rust
fn transform_collection<F, T>(collection: Vec<T>, transform: F) -> Vec<T>
where
    F: for<'a> Fn(&'a T) -> T,
    T: Clone,
{
    collection.iter().map(|x| transform(x)).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let transformed_numbers = transform_collection(numbers.clone(), |x| x * 2);
    println!("Original Numbers: {:?}", numbers);
    println!("Transformed Numbers: {:?}", transformed_numbers);

    let words = vec!["apple", "banana", "cherry"];
    let transformed_words = transform_collection(words.clone(), |word| {
        // error
        format!("{}!", word)
    });
    println!("Original Words: {:?}", words);
    println!("Transformed Words: {:?}", transformed_words);
}
```