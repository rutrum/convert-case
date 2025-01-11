use unicode_segmentation::UnicodeSegmentation;

fn grapheme_is_digit(c: &str) -> bool {
    c.chars().all(|c| c.is_ascii_digit())
}

fn grapheme_is_uppercase(c: &str) -> bool {
    c.to_uppercase() != c.to_lowercase() && c == c.to_uppercase()
}

fn grapheme_is_lowercase(c: &str) -> bool {
    c.to_uppercase() != c.to_lowercase() && c == c.to_lowercase()
}

#[derive(Debug, Eq, Hash, Clone, Copy)]
pub struct Boundary {
    name: &'static str,
    condition: fn(&str) -> bool,
    start: usize,
    len: usize,
}

impl PartialEq for Boundary {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Boundary {
    // TODO maybe use graphemes here
    pub const SPACE: Boundary = Boundary {
        name: "Space",
        condition: |s| s.graphemes(true).next() == Some(" "),
        start: 0,
        len: 1,
    };
    pub const HYPHEN: Boundary = Boundary {
        name: "Hyphen",
        condition: |s| s.graphemes(true).next() == Some("-"),
        start: 0,
        len: 1,
    };
    pub const UNDERSCORE: Boundary = Boundary {
        name: "Underscore",
        condition: |s| s.graphemes(true).next() == Some("_"),
        start: 0,
        len: 1,
    };
    pub const LOWER_UPPER: Boundary = Boundary {
        name: "LowerUpper",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_lowercase).unwrap_or(false)
                && chars.next().map(grapheme_is_uppercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const ACRONYM: Boundary = Boundary {
        name: "Acronym",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_uppercase).unwrap_or(false)
                && chars.next().map(grapheme_is_uppercase).unwrap_or(false)
                && chars.next().map(grapheme_is_lowercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };

    // less used
    pub const UPPER_LOWER: Boundary = Boundary {
        name: "UpperLower",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_uppercase).unwrap_or(false)
                && chars.next().map(grapheme_is_lowercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };

    // digits
    pub const LOWER_DIGIT: Boundary = Boundary {
        name: "LowerDigit",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_lowercase).unwrap_or(false)
                && chars.next().map(grapheme_is_digit).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const UPPER_DIGIT: Boundary = Boundary {
        name: "UpperDigit",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_uppercase).unwrap_or(false)
                && chars.next().map(grapheme_is_digit).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const DIGIT_LOWER: Boundary = Boundary {
        name: "DigitLower",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_digit).unwrap_or(false)
                && chars.next().map(grapheme_is_lowercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const DIGIT_UPPER: Boundary = Boundary {
        name: "DigitUpper",
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_digit).unwrap_or(false)
                && chars.next().map(grapheme_is_uppercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };

    pub const fn defaults() -> [Boundary; 9] {
        [
            Boundary::SPACE,
            Boundary::HYPHEN,
            Boundary::UNDERSCORE,
            Boundary::LOWER_UPPER,
            Boundary::ACRONYM,
            Boundary::LOWER_DIGIT,
            Boundary::UPPER_DIGIT,
            Boundary::DIGIT_LOWER,
            Boundary::DIGIT_UPPER,
        ]
    }

    pub const fn digits() -> [Boundary; 4] {
        [
            Boundary::LOWER_DIGIT,
            Boundary::UPPER_DIGIT,
            Boundary::DIGIT_LOWER,
            Boundary::DIGIT_UPPER,
        ]
    }

    pub fn list_from(pattern: &str) -> Vec<Boundary> {
        let mut boundaries = Vec::new();
        for boundary in Boundary::defaults() {
            let parts = split(&pattern, &[boundary]);
            if parts.len() > 1 || parts[0] != pattern {
                boundaries.push(boundary);
            }
        }
        boundaries
    }
}

// another idea for this algorithm
// build an array of integers where
// 0 means no split
// 1 means the split is left of this char
// 2 means this character is removed
// then I can build the word at the end
pub fn split<'s, T>(s: &'s T, boundaries: &[Boundary]) -> Vec<&'s str>
where
    T: AsRef<str>,
{
    let s = s.as_ref();

    if s.len() == 0 {
        return vec![];
    }

    let mut words = Vec::new();
    let mut last_boundary_end = 0;

    let (indices, graphemes): (Vec<_>, Vec<_>) = s.grapheme_indices(true).unzip();
    let grapheme_length = indices[graphemes.len() - 1] + graphemes[graphemes.len() - 1].len();

    for i in 0..graphemes.len() {
        for boundary in boundaries {
            let byte_index = indices[i];

            if (boundary.condition)(&s[byte_index..]) {
                // What if we find a condition at the end of the array?
                // Maybe we can stop early based on length
                // To do this, need to switch the loops
                // TODO
                let boundary_byte_start: usize =
                    *indices.get(i + boundary.start).unwrap_or(&grapheme_length);
                let boundary_byte_end: usize = *indices
                    .get(i + boundary.start + boundary.len)
                    .unwrap_or(&grapheme_length);

                // todo clean this up a bit
                words.push(&s[last_boundary_end..boundary_byte_start]);
                last_boundary_end = boundary_byte_end;
                break;
            }
        }
    }
    words.push(&s[last_boundary_end..]);
    words.into_iter().filter(|s| !s.is_empty()).collect() // this filter breaks boundary checking
}

// ascii version
//pub fn split<'s, T>(s: &'s T, boundaries: &[Boundary]) -> Vec<&'s str>
//where
//    T: AsRef<str>,
//{
//    let s = s.as_ref();
//
//    let mut words = Vec::new();
//    let mut last_end = 0;
//    for i in 0..s.len() {
//        for boundary in boundaries {
//            if (boundary.condition)(&s[i..]) {
//                words.push(&s[last_end..i + boundary.start]);
//                last_end = i + boundary.start + boundary.len;
//                break;
//            }
//        }
//    }
//    words.push(&s[last_end..]);
//    words
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyphen() {
        let s = "a-b-c";
        let v = split(&s, &[Boundary::HYPHEN]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn underscore() {
        let s = "a_b_c";
        let v = split(&s, &[Boundary::UNDERSCORE]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn space() {
        let s = "a b c";
        let v = split(&s, &[Boundary::SPACE]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn delimiters() {
        let s = "aaa-bbb_ccc ddd ddd-eee";
        let v = split(
            &s,
            &[Boundary::SPACE, Boundary::UNDERSCORE, Boundary::HYPHEN],
        );
        assert_eq!(v, vec!["aaa", "bbb", "ccc", "ddd", "ddd", "eee"]);
    }

    #[test]
    fn lower_upper() {
        let s = "lowerUpperUpper";
        let v = split(&s, &[Boundary::LOWER_UPPER]);
        assert_eq!(v, vec!["lower", "Upper", "Upper"]);
    }

    #[test]
    fn acronym() {
        let s = "XMLRequest";
        let v = split(&s, &[Boundary::ACRONYM]);
        assert_eq!(v, vec!["XML", "Request"]);
    }

    // TODO: add tests for other boundaries

    #[test]
    fn boundaries_found_in_string() {
        // upper lower is not longer a default
        assert_eq!(Vec::<Boundary>::new(), Boundary::list_from(".Aaaa"));
        assert_eq!(
            vec![Boundary::LOWER_UPPER, Boundary::LOWER_DIGIT,],
            Boundary::list_from("a8.Aa.aA")
        );
        assert_eq!(Boundary::digits().to_vec(), Boundary::list_from("b1B1b"));
        assert_eq!(
            vec![
                Boundary::SPACE,
                Boundary::HYPHEN,
                Boundary::UNDERSCORE,
                Boundary::ACRONYM,
            ],
            Boundary::list_from("AAa -_")
        );
    }

    #[test]
    fn boundary_consts_same() {
        assert_eq!(Boundary::SPACE, Boundary::SPACE);
    }
}
