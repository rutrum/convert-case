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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Boundary {
    condition: fn(&str) -> bool,
    start: usize,
    len: usize,
}

impl Boundary {
    // TODO maybe use graphemes here
    pub const SPACE: Boundary = Boundary {
        condition: |s| s.graphemes(true).next() == Some(" "),
        start: 0,
        len: 1,
    };
    pub const HYPHEN: Boundary = Boundary {
        condition: |s| s.graphemes(true).next() == Some("-"),
        start: 0,
        len: 1,
    };
    pub const UNDERSCORE: Boundary = Boundary {
        condition: |s| s.graphemes(true).next() == Some("_"),
        start: 0,
        len: 1,
    };
    pub const LOWER_UPPER: Boundary = Boundary {
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_lowercase).unwrap_or(false)
                && chars.next().map(grapheme_is_uppercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const ACRONYM: Boundary = Boundary {
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
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_lowercase).unwrap_or(false)
                && chars.next().map(grapheme_is_digit).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const UPPER_DIGIT: Boundary = Boundary {
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_uppercase).unwrap_or(false)
                && chars.next().map(grapheme_is_digit).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const DIGIT_LOWER: Boundary = Boundary {
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_digit).unwrap_or(false)
                && chars.next().map(grapheme_is_lowercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const DIGIT_UPPER: Boundary = Boundary {
        condition: |s| {
            let mut chars = s.graphemes(true);
            chars.next().map(grapheme_is_digit).unwrap_or(false)
                && chars.next().map(grapheme_is_uppercase).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };

    // todo remove?
    pub const fn default_delimiters() -> [Boundary; 3] {
        [Boundary::SPACE, Boundary::HYPHEN, Boundary::UNDERSCORE]
    }

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

    let mut words = Vec::new();
    let mut last_end = 0;

    let (indices, graphemes): (Vec<_>, Vec<_>) = s.grapheme_indices(true).unzip();

    for i in 0..graphemes.len() {
        for boundary in boundaries {
            let byte_index = indices[i];

            if (boundary.condition)(&s[byte_index..]) {
                let boundary_byte_start: usize = *indices
                    .get(i + boundary.start)
                    .unwrap_or(&(graphemes.len()));
                //let boundary_byte_end: usize = indices.get(i + boundary.start + boundary.len);
                let boundary_byte_end: usize = *indices
                    .get(i + boundary.start + boundary.len)
                    .unwrap_or(&(graphemes.len()));

                // todo clean this up a bit
                words.push(&s[last_end..boundary_byte_start]);
                last_end = boundary_byte_end;
                break;
            }
        }
    }
    words.push(&s[last_end..]);
    words.into_iter().filter(|s| !s.is_empty()).collect()
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
        let v = split(&s, &Boundary::default_delimiters());
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
}
