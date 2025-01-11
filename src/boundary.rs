pub struct Boundary {
    condition: fn(&str) -> bool,
    start: usize,
    len: usize,
}

impl Boundary {
    // TODO maybe use graphemes here
    pub const SPACE: Boundary = Boundary {
        condition: |s| s.chars().nth(0) == Some(' '),
        start: 0,
        len: 1,
    };
    pub const HYPHEN: Boundary = Boundary {
        condition: |s| s.chars().nth(0) == Some('-'),
        start: 0,
        len: 1,
    };
    pub const UNDERSCORE: Boundary = Boundary {
        condition: |s| s.chars().nth(0) == Some('_'),
        start: 0,
        len: 1,
    };
    pub const LOWER_UPPER: Boundary = Boundary {
        condition: |s| {
            let mut chars = s.chars();
            chars.next().map(|c| c.is_lowercase()).unwrap_or(false)
                && chars.next().map(|c| c.is_uppercase()).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };
    pub const ACRONYM: Boundary = Boundary {
        condition: |s| {
            let mut chars = s.chars();
            chars.next().map(|c| c.is_uppercase()).unwrap_or(false)
                && chars.next().map(|c| c.is_uppercase()).unwrap_or(false)
                && chars.next().map(|c| c.is_lowercase()).unwrap_or(false)
        },
        start: 1,
        len: 0,
    };

    pub const fn default_delimiters() -> [Boundary; 3] {
        [Boundary::SPACE, Boundary::HYPHEN, Boundary::UNDERSCORE]
    }
}

// another idea for this algorithm
// build an array of integers where
// 0 means no split
// 1 means the split is left of this char
// 2 means this character is removed
// then I can build the word at the end
fn split<'s>(s: &'s str, boundaries: &[Boundary]) -> Vec<&'s str> {
    let mut words = Vec::new();
    let mut last_end = 0;
    for i in 0..s.len() {
        for boundary in boundaries {
            if (boundary.condition)(&s[i..]) {
                words.push(&s[last_end..i + boundary.start]);
                last_end = i + boundary.start + boundary.len;
                break;
            }
        }
    }
    words.push(&s[last_end..]);
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hyphen() {
        let s = "a-b-c";
        let v = split(s, &[Boundary::HYPHEN]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn underscore() {
        let s = "a_b_c";
        let v = split(s, &[Boundary::UNDERSCORE]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn space() {
        let s = "a b c";
        let v = split(s, &[Boundary::SPACE]);
        assert_eq!(v, vec!["a", "b", "c"]);
    }

    #[test]
    fn delimiters() {
        let s = "aaa-bbb_ccc ddd ddd-eee";
        let v = split(s, &Boundary::default_delimiters());
        assert_eq!(v, vec!["aaa", "bbb", "ccc", "ddd", "ddd", "eee"]);
    }

    #[test]
    fn lower_upper() {
        let s = "lowerUpperUpper";
        let v = split(s, &[Boundary::LOWER_UPPER]);
        assert_eq!(v, vec!["lower", "Upper", "Upper"]);
    }

    #[test]
    fn acronym() {
        let s = "XMLRequest";
        let v = split(s, &[Boundary::ACRONYM]);
        assert_eq!(v, vec!["XML", "Request"]);
    }
}
