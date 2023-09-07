fn main() {
    let found = "στιγμας🙂";
    let suggested = "στιγμασ😃";

    // let max_offset = 1; // Maximum allowed character offset

    // let found_chars = found.chars().collect::<Vec<_>>();
    // let suggested_chars = suggested.chars().collect::<Vec<_>>();

    // Check if the strings are visually similar with character offset
    let similar = found
        .chars()
        .collect::<Vec<_>>()
        .iter()
        .zip(suggested.chars().collect::<Vec<_>>().iter())
        .all(|(f, s)| (*f as i32 - *s as i32).abs() <= 1 && f.to_lowercase().eq(s.to_lowercase()));

    if similar
        && (suggested.chars().collect::<Vec<_>>().len()
            == suggested.chars().collect::<Vec<_>>().len())
    {
        println!("Strings are SIMILAR with character offset.");
    } else {
        println!("Strings are DIFFERENT.");
    }
}
