/// Returns the assembled code suggestions, whether they should be shown with an underline
/// and whether the substitution only differs in capitalization.
pub fn splice_lines(lines: &[String], underline: bool, capitalization_diff: bool) -> Vec<String> {
    let mut result = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if underline {
            result.push(format!(
                "{} {}",
                if index == 0 { "^" } else { " " },
                line
            ));
        } else {
            result.push(line.to_string());
        }

        if capitalization_diff {
            let capitalized_line = line.chars().enumerate().map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().collect::<String>()
                } else {
                    c.to_lowercase().collect::<String>()
                }
            }).collect::<String>();

            result.push(format!("Did you mean '{}'?", capitalized_line));
        }
    }

    result
}

fn main() {
    let input_lines = vec![
        "fn Main() {".to_string(),
        "    println!(\"hello, world!\");".to_string(),
        "}".to_string(),
    ];

    let underline = true;
    let capitalization_diff = true;

    let suggestions = splice_lines(&input_lines, underline, capitalization_diff);

    for suggestion in &suggestions {
        println!("{}", suggestion);
    }
}
