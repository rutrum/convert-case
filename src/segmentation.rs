/// A boundary defines how a string is split into words.  Some boundaries, `Hyphen`, `Underscore`,
/// and `Space`, consume the character they split on, whereas the other boundaries
/// do not.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Boundary {
    /// Splits on `-`, consuming the character on segmentation.
    Hyphen,

    /// Splits on `_`, consuming the character on segmentation.
    Underscore,

    /// Splits on space, consuming the character on segmentation.
    Space,

    /// Splits where an uppercase letter is followed by a lowercase letter.  This is seldom used,
    /// and is not included in the [defaults](Boundary::defaults).
    UpperLower,

    /// Splits where a lowercase letter is followed by an uppercase letter.
    LowerUpper,

    /// Splits where digit is followed by an uppercase letter.
    DigitUpper,

    /// Splits where an uppercase letter is followed by a digit.
    UpperDigit,

    /// Splits where digit is followed by a lowercase letter.
    DigitLower,

    /// Splits where a lowercase letter is followed by a digit.
    LowerDigit,

    /// Acronyms are identified by two uppercase letters followed by a lowercase letter.
    /// The word boundary is between the two uppercase letters.  For example, "HTTPRequest"
    /// would have an acronym boundary identified at "PRe" and split into "HTTP" and "Request".
    Acronyms, // rename to acronym
}

impl Boundary {
    /// Returns the boundaries that split around single characters, `Hyphen`,
    /// `Underscore`, and `Space`.
    pub fn delims() -> Vec<Self> {
        use Boundary::*;
        vec![Hyphen, Underscore, Space]
    }

    /// Returns the boundaries that involve digits, `DigitUpper`, `DigitLower`, `UpperDigit`, and
    /// `LowerDigit`.
    pub fn digits() -> Vec<Self> {
        use Boundary::*;
        vec![DigitUpper, DigitLower, UpperDigit, LowerDigit]
    }

    /// The default list of boundaries used when no case is provided in `from_case`.  This includes
    /// all the boundaries except the `UpperLower` boundary.
    pub fn defaults() -> Vec<Self> {
        use Boundary::*;
        vec![
            Underscore, Hyphen, Space, LowerUpper, UpperDigit, DigitUpper, DigitLower, LowerDigit,
            Acronyms,
        ]
    }

    fn detect_one(&self, c: char) -> bool {
        use Boundary::*;
        match self {
            Hyphen => c == '-',
            Underscore => c == '_',
            Space => c == ' ',
            _ => false,
        }
    }

    fn detect_two(&self, c: char, d: char) -> bool {
        use Boundary::*;
        match self {
            UpperLower => c.is_uppercase() && d.is_lowercase(),
            LowerUpper => c.is_lowercase() && d.is_uppercase(),
            DigitUpper => c.is_ascii_digit() && d.is_uppercase(),
            UpperDigit => c.is_uppercase() && d.is_ascii_digit(),
            DigitLower => c.is_ascii_digit() && d.is_lowercase(),
            LowerDigit => c.is_lowercase() && d.is_ascii_digit(),
            _ => false,
        }
    }

    fn detect_three(&self, c: char, d: char, e: char) -> bool {
        use Boundary::*;
        if let Acronyms = self {
            c.is_uppercase() && d.is_uppercase() && e.is_lowercase()
        } else {
            false
        }
    }
}

// idea: make a bitset for each boundary.  Its fixed size,
// and can be copied.  Also no fear in adding duplicates

// gross
pub fn split<'a, T: ?Sized>(s: &'a T, boundaries: &[Boundary]) -> Vec<&'a str>
where
    T: AsRef<str>,
{
    let s = s.as_ref();

    let single_splits = s
        .chars()
        .enumerate()
        .filter(|(_, c)| boundaries.iter().any(|b| b.detect_one(*c)))
        .map(|(i, _)| i + 1)
        .collect();

    let words = replace_at_indicies(s, single_splits);

    let final_words = words.iter().flat_map(|&w| {
        let left_iter = w.chars();
        let mid_iter = w.chars().skip(1);
        let right_iter = w.chars().skip(2);

        let three_iter = left_iter.clone().zip(mid_iter.clone()).zip(right_iter);
        let two_iter = left_iter.clone().zip(mid_iter);

        let mut splits: Vec<usize> = three_iter
            .enumerate()
            .filter(|(_, ((c, d), e))| boundaries.iter().any(|b| b.detect_three(*c, *d, *e)))
            .map(|(i, _)| i + 1)
            .chain(
                two_iter
                    .enumerate()
                    .filter(|(_, (c, d))| boundaries.iter().any(|b| b.detect_two(*c, *d)))
                    .map(|(i, _)| i + 1),
            )
            .collect();
        splits.sort_unstable();

        split_on_indicies(w, splits)
    });

    final_words.rev().filter(|s| !s.is_empty()).collect()
}

pub fn replace_at_indicies(s: &str, splits: Vec<usize>) -> Vec<&str> {
    let mut words = Vec::new();

    let mut first = s;
    let mut second;
    for &x in splits.iter().rev() {
        let pair = first.split_at(x);
        first = &pair.0[..(pair.0.len() - 1)];
        second = pair.1;
        words.push(second);
    }
    words.push(first);

    words
}

pub fn split_on_indicies(s: &str, splits: Vec<usize>) -> Vec<&str> {
    let mut words = Vec::new();

    let mut first = s;
    let mut second;
    for &x in splits.iter().rev() {
        let pair = first.split_at(x);
        first = pair.0;
        second = pair.1;
        words.push(second);
    }
    words.push(first);

    words
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_on_delims() {
        assert_eq!(
            vec!["my", "word", "list", "separated", "by", "delims"],
            split("my_word-list separated-by_delims", &Boundary::delims())
        )
    }
}
