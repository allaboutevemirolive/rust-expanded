struct SourceMap;

#[derive(Debug)]
struct Span;

impl SourceMap {
    fn span_to_snippet(&self, _sp: &Span, ic: &str) -> Result<String, String> {
        Ok(ic.to_string())
    }
}

fn is_case_difference(sm: &SourceMap, suggested: &str, sp: &Span, initial_code: &str) -> bool {
    // Retrieve original code
    let found = match sm.span_to_snippet(sp, initial_code) {
        Ok(snippet) => {
            println!("{:?}", snippet);
            snippet
        }
        Err(_) => return false,
    };

    // Compare normalized strings for case differences
    found.chars().zip(suggested.chars()).any(|(found_char, suggested_char)| {
        found_char != suggested_char &&
        found_char.to_lowercase().eq(suggested_char.to_lowercase())
    }) || found != suggested
}


fn main() {
    let sm = SourceMap;
    let initial_code = "Ætheric landscapes hold an œuvre of mystical beauty.";
    let suggested = "Ætheric landscapes hold an œuvre of mystical beauty.";
    let sp = Span;

    if is_case_difference(&sm, suggested, &sp, &initial_code) {
        println!("comparison is DIFFERENT");
    } else {
        println!("comparison is SAME");
    }
}