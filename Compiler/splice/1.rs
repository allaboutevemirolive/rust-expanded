fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Splice in new elements at index 2
    let new_elements = vec![10, 11, 12];
    numbers.splice(2..3, new_elements);

    println!("After splicing: {:?}", numbers);
}
