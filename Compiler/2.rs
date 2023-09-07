#[derive(Debug, Clone)]
struct Span {
    lo: usize,
    hi: usize,
}

impl Span {
    fn with_root_ctxt(lo: usize, hi: usize) -> Self {
        Span { lo, hi }
    }
}

#[derive(Debug, Clone)]
struct Part {
    span: Span,
}

fn main() {
    // Example input data: sorted spans for demonstration
    let substitution_parts = vec![
        Part { span: Span { lo: 5, hi: 10 } },
        Part { span: Span { lo: 15, hi: 20 } },
        Part { span: Span { lo: 25, hi: 30 } },
    ];

    // Sort the spans in ascending order based on start location
    let mut sorted_parts = substitution_parts.clone();
    sorted_parts.sort_by_key(|part| part.span.lo);

    // Find the bounding span
    let lo = sorted_parts.iter().map(|part| part.span.lo).min().unwrap_or(0);
    let hi = sorted_parts.iter().map(|part| part.span.hi).max().unwrap_or(0);
    let bounding_span = Span::with_root_ctxt(lo, hi);

    println!("Bounding Span: {:?}", bounding_span);
}
