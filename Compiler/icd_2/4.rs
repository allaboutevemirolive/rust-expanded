
use core::iter;
struct SourceMap;

#[derive(Debug)]
struct Span;

impl SourceMap {
    fn span_to_snippet(&self, _sp: &Span, ic: &str) -> Result<String, String> {
        Ok(ic.to_string())
    }
}

fn is_case_difference(sm: &SourceMap, suggested: &str, sp: &Span, initial_code: &str) -> bool {
    if let Ok(found) = sm.span_to_snippet(sp, initial_code) {
        const ASCII_CONFUSABLES: &[char] = &['c', 'f', 'i', 'k', 'o', 's', 'u', 'v', 'w', 'x', 'y', 'z'];

        found.chars()
            .zip(suggested.chars())
            .all(|(f, s)| f == s || ASCII_CONFUSABLES.contains(&f) || ASCII_CONFUSABLES.contains(&s))
            && found.eq_ignore_ascii_case(suggested)
            && found != suggested
    } else {
        false
    }
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



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_1() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let FOO = 42;", &sp, "let Foo = 42;"),
            true
        );
    }

    #[test]
    fn test_case_2() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let foo bar = 42;", &sp, "let FoO BAR = 42;"),
            true
        );
    }

    #[test]
    fn test_case_3() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let foo bar = 42;", &sp, "let foo bar = 42;"),
            false
        );
    }

    #[test]
    fn test_case_4() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let στιγμας = 42;", &sp, "let στιγμασ = 42;"),
            true
        );
    }

    #[test]
    fn test_case_5() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let στιγμας = 42;", &sp, "let στιγμας = 42;"),
            false
        );
    }

    #[test]
    fn test_case_6() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let FO_ = 42;", &sp, "let Foo = 42;"),
            true
        );
    }

    #[test]
    fn test_case_7() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let FOO = 42;", &sp, "let Foo = 42;"),
            true
        );
    }

    #[test]
    fn test_case_8() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let ﬂour = 42;", &sp, "let flour = 42;"),
            true
        );
    }

    #[test]
    fn test_case_9() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let flour = 42;", &sp, "let flour = 42;"),
            false
        );
    }

    #[test]
    fn test_case_10() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let Maße = 42;", &sp, "let MASSE = 42;"),
            true
        );
    }

    #[test]
    fn test_case_11() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let Maße = 42;", &sp, "let Maße = 42;"),
            false
        );
    }

    #[test]
    fn test_case_12() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(
                &sm,
                "let ᾲ στο διάολο = 42;",
                &sp,
                "let ὰι στο διάολο = 42;"
            ),
            true
        );
    }

    #[test]
    fn test_case_13() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let ᾲ στο διάολο = 42;", &sp, "let ᾲ στο διάολο = 42;"),
            false
        );
    }

    #[test]
    fn test_case_14() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let FO0 = 42;", &sp, "let FOO = 42;"),
            true
        );
    }

    #[test]
    fn test_case_15() {
        let sm = SourceMap;
        let sp = Span;
        assert_eq!(
            is_case_difference(&sm, "let FO0 = 42;", &sp, "let FO0 = 42;"),
            false
        );
    }

    #[test]
    fn test_case_16() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let a123 = 42;", &sp, "LET A12₃ = 42;"),
            true
        );
    }

    #[test]
    fn test_case_17() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let A12₃ = 42;", &sp, "let A12₃ = 42;"),
            false
        );
    }

    #[test]
    fn test_case_18() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let Café = 42;", &sp, "let CAFÉ = 42;"),
            true
        );
    }

    #[test]
    fn test_case_19() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let CAFÉ = 42;", &sp, "let CAFÉ = 42;"),
            false
        );
    }

    #[test]
    fn test_case_20() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let ABC123 = 42;", &sp, "let abc456 = 42;"),
            true
        );
    }

    #[test]
    fn test_case_21() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let ABC123 = 42;", &sp, "let ABC123 = 42;"),
            false
        );
    }

    #[test]
    fn test_case_22() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let l_1O = 42;", &sp, "let I_10 = 42;"),
            true
        );
    }

    #[test]
    fn test_case_23() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let l_1O = 42;", &sp, "let l_1O = 42;"),
            false
        );
    }

    #[test]
    fn test_case_24() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let foo_bar_BAZ = 42;", &sp, "let FOO BAR BAZ = 42;"),
            true
        );
    }

    #[test]
    fn test_case_25() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let ΑΒΓΔ = 42;", &sp, "let αβγδ = 42;"),
            true
        );
    }

    #[test]
    fn test_case_26() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let OoOsS = 42;", &sp, "let 00O5s = 42;"),
            true
        );
    }

    #[test]
    fn test_case_27() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let 00O5s = 42;", &sp, "let 00O5s = 42;"),
            false
        );
    }

    #[test]
    fn test_case_28() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let áḇćd = 42;", &sp, "let àḃcd = 42;"),
            true
        );
    }

    #[test]
    fn test_case_29() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let áḇćd = 42;", &sp, "let áḇćd = 42;"),
            false
        );
    }

    #[test]
    fn test_case_30() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let 𝕬𝖭 = 42;", &sp, "let 𝑨𝑵 = 42;"),
            true
        );
    }

    #[test]
    fn test_case_31() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let Hello! = 42;", &sp, "let hello? = 42;"),
            true
        );
    }

    #[test]
    fn test_case_32() {
        let sm = SourceMap;
        let sp = Span;

        assert_eq!(
            is_case_difference(&sm, "let Hello! = 42;", &sp, "let Hello! = 42;"),
            false
        );
    }
}

