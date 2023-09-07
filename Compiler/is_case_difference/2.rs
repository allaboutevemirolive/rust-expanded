fn main() {
    let found = "στιγμα";
    let suggested = "στιγμα";

    let max_offset = 1; // Maximum allowed character offset

    let found_chars = found.chars().collect::<Vec<_>>();
    let suggested_chars = suggested.chars().collect::<Vec<_>>();

    // Check if the strings are visually similar with character offset
    let similar = found_chars.iter().zip(suggested_chars.iter()).all(|(f, s)| {
        let offset = (*f as i32 - *s as i32).abs();
        offset <= max_offset && f.to_lowercase().eq(s.to_lowercase())
    });

    if similar && (found_chars.len() == suggested_chars.len()) {
        println!("Strings are SIMILAR with character offset.");
    } else {
        println!("Strings are DIFFERENT.");
    }
}
