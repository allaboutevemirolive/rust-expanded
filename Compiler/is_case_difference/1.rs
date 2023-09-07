fn main() {
    let found = "στιγμα";
    let suggested = "στιγμα";

    // let max_offset = 1; // Maximum allowed character offset

    // Check if the strings are visually similar with character offset
    let similar = found.char_indices().zip(suggested.char_indices()).all(|((idx_f, f), (idx_s, s))| {
        let offset = (idx_f as i32 - idx_s as i32).abs();
        offset <= 1 && f.eq_ignore_ascii_case(&s)
    });

    if similar && (found.chars().count() == suggested.chars().count()) {
        println!("Strings are SIMILAR with character offset.");
    } else {
        println!("Strings are DIFFERENT.");
    }
}
