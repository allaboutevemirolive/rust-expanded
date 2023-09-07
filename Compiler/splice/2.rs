fn main() {
    let mut source_code = "fn main() {\n    println!(\"Hello, world!\");\n}\n".to_string();

    // Splice in a new line at line index 1
    let new_line = "    // This line was spliced in";
    let mut lines: Vec<_> = source_code.lines().collect();
    let index_to_splice = 1;

    if index_to_splice < lines.len() {
        lines.insert(index_to_splice, new_line);
        source_code = lines.join("\n");
    } else {
        println!("Index out of bounds.");
    }

    println!("After splicing:\n{}", source_code);
}
