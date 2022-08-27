#[cfg(test)]
use strum_macros::EnumIter;

use unicode_segmentation::UnicodeSegmentation;

/// A boundary defines how a string is split into words.  Some boundaries, `Hyphen`, `Underscore`,
/// and `Space`, consume the character they split on, whereas the other boundaries
/// do not.
///
/// The struct offers methods that return `Vec`s containing useful groups of boundaries.  It also
/// contains the [`list_from`](Boundary::list_from) method which will generate a list of boundaries
/// based on a string slice.
///
/// Note that all boundaries are distinct and do not share functionality.  That is, there is no
/// such DigitLetter variant, because that would be equivalent to the current `DigitUpper` and
/// `DigitLower` variants.  For common functionality, consider using
/// some provided functions that return a list of boundaries.
/// ```
/// use convert_case::{Boundary, Case, Casing, Converter};
///
/// assert_eq!(
///     "transformations_in_3d",
///     "TransformationsIn3D"
///         .from_case(Case::Camel)
///         .without_boundaries(&Boundary::digit_letter())
///         .to_case(Case::Snake)
/// );
///
/// let conv = Converter::new()
///     .set_boundaries(&Boundary::list_from("aA "))
///     .to_case(Case::Title);
/// assert_eq!("7empest By Tool", conv.convert("7empest byTool"));
/// ```
#[cfg_attr(test, derive(EnumIter))]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Boundary {
    /// Splits on `-`, consuming the character on segmentation.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::Hyphen],
    ///     Boundary::list_from("-")
    /// );
    /// ```
    Hyphen,

    /// Splits on `_`, consuming the character on segmentation.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::Underscore],
    ///     Boundary::list_from("_")
    /// );
    /// ```
    Underscore,

    /// Splits on space, consuming the character on segmentation.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::Space],
    ///     Boundary::list_from(" ")
    /// );
    /// ```
    Space,

    /// Splits where an uppercase letter is followed by a lowercase letter.  This is seldom used,
    /// and is not included in the [defaults](Boundary::defaults).
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::UpperLower],
    ///     Boundary::list_from("Aa")
    /// );
    /// ```
    UpperLower,

    /// Splits where a lowercase letter is followed by an uppercase letter.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::LowerUpper],
    ///     Boundary::list_from("aA")
    /// );
    /// ```
    LowerUpper,

    /// Splits where digit is followed by an uppercase letter.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::DigitUpper],
    ///     Boundary::list_from("1A")
    /// );
    /// ```
    DigitUpper,

    /// Splits where an uppercase letter is followed by a digit.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::UpperDigit],
    ///     Boundary::list_from("A1")
    /// );
    /// ```
    UpperDigit,

    /// Splits where digit is followed by a lowercase letter.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::DigitLower],
    ///     Boundary::list_from("1a")
    /// );
    /// ```
    DigitLower,

    /// Splits where a lowercase letter is followed by a digit.
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::LowerDigit],
    ///     Boundary::list_from("a1")
    /// );
    /// ```
    LowerDigit,

    /// Acronyms are identified by two uppercase letters followed by a lowercase letter.
    /// The word boundary is between the two uppercase letters.  For example, "HTTPRequest"
    /// would have an acronym boundary identified at "PRe" and split into "HTTP" and "Request".
    /// ```
    /// use convert_case::Boundary;
    /// assert_eq!(
    ///     vec![Boundary::Acronym],
    ///     Boundary::list_from("AAa")
    /// );
    /// ```
    Acronym,
}

impl Boundary {
    /// Returns a list of all boundaries that are identified within the given string.
    /// Could be a short of writing out all the boundaries in a list directly.  This will not
    /// identify boundary `UpperLower` if it also used as part of `Acronym`.
    ///
    /// If you want to be very explicit and not overlap boundaries, it is recommended to use a colon
    /// character.
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![Hyphen, Space, LowerUpper, UpperDigit, DigitLower],
    ///     Boundary::list_from("aA8a -")
    /// );
    /// assert_eq!(
    ///     vec![Underscore, LowerUpper, DigitUpper, Acronym],
    ///     Boundary::list_from("bD:0B:_:AAa")
    /// );
    /// ```
    pub fn list_from(s: &str) -> Vec<Self> {
        Boundary::all().iter().filter(|boundary| {
            let left_iter = s.graphemes(true);
            let mid_iter = s.graphemes(true).skip(1);
            let right_iter = s.graphemes(true).skip(2);

            let mut one_iter = left_iter.clone();

            // Also capture when the previous pair was both uppercase, so we don't
            // match the UpperLower boundary in the case of Acronym
            let two_iter = left_iter.clone().zip(mid_iter.clone());
            let mut two_iter_and_upper = two_iter.clone()
                .zip(std::iter::once(false).chain(
                        two_iter.map(|(a, b)| a == a.to_uppercase() && b == b.to_uppercase())
                ));

            let mut three_iter = left_iter.zip(mid_iter).zip(right_iter);

            one_iter.any(|a| boundary.detect_one(a))
                || two_iter_and_upper.any(|((a, b), is_acro)| boundary.detect_two(a, b) && !is_acro)
                || three_iter.any(|((a, b), c)| boundary.detect_three(a, b, c))
        }).map(|b| *b).collect()
    }

    /// The default list of boundaries used when `Casing::to_case` is called directly
    /// and in a `Converter` generated from `Converter::new()`.  This includes
    /// all the boundaries except the `UpperLower` boundary.
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![
    ///         Underscore, Hyphen, Space, LowerUpper, UpperDigit, 
    ///         DigitUpper, DigitLower, LowerDigit, Acronym,
    ///     ],
    ///     Boundary::defaults()
    /// );
    /// ```
    pub fn defaults() -> Vec<Self> {
        use Boundary::*;
        vec![
            Underscore, Hyphen, Space, LowerUpper, UpperDigit, DigitUpper, DigitLower, LowerDigit,
            Acronym,
        ]
    }

    /// Returns the boundaries that split around single characters: `Hyphen`,
    /// `Underscore`, and `Space`.
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![Hyphen, Underscore, Space],
    ///     Boundary::delims()
    /// );
    /// ```
    pub fn delims() -> Vec<Self> {
        use Boundary::*;
        vec![Hyphen, Underscore, Space]
    }

    /// Returns the boundaries that involve digits: `DigitUpper`, `DigitLower`, `UpperDigit`, and
    /// `LowerDigit`.
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![DigitUpper, UpperDigit, DigitLower, LowerDigit],
    ///     Boundary::digits()
    /// );
    /// ```
    pub fn digits() -> Vec<Self> {
        use Boundary::*;
        vec![DigitUpper, UpperDigit, DigitLower, LowerDigit]
    }

    /// Returns the boundaries that are letters followed by digits: `UpperDigit` and `LowerDigit`.
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![UpperDigit, LowerDigit],
    ///     Boundary::letter_digit()
    /// );
    /// ```
    pub fn letter_digit() -> Vec<Self> {
        use Boundary::*;
        vec![UpperDigit, LowerDigit]
    }

    /// Returns the boundaries that are digits followed by letters: `DigitUpper` and
    /// `DigitLower`.
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![DigitUpper, DigitLower],
    ///     Boundary::digit_letter()
    /// );
    /// ```
    pub fn digit_letter() -> Vec<Self> {
        use Boundary::*;
        vec![DigitUpper, DigitLower]
    }

    /// Returns all boundaries.  Note that this includes the `UpperLower` variant which
    /// might be unhelpful.  Please look at [`Boundary::defaults`].
    /// ```
    /// use convert_case::Boundary;
    /// use Boundary::*;
    /// assert_eq!(
    ///     vec![
    ///         Hyphen, Underscore, Space, LowerUpper, UpperLower, DigitUpper,
    ///         UpperDigit, DigitLower, LowerDigit, Acronym,
    ///     ],
    ///     Boundary::all()
    /// );
    /// ```
    pub fn all() -> Vec<Self> {
        use Boundary::*;
        vec![
            Hyphen, Underscore, Space, LowerUpper, UpperLower, DigitUpper, UpperDigit, 
            DigitLower, LowerDigit, Acronym
        ]
    }

    fn detect_one(&self, c: &str) -> bool {
        use Boundary::*;
        match self {
            Hyphen => c == "-",
            Underscore => c == "_",
            Space => c == " ",
            _ => false,
        }
    }

    fn detect_two(&self, c: &str, d: &str) -> bool {
        use Boundary::*;
        match self {
            UpperLower => grapheme_is_uppercase(c) && grapheme_is_lowercase(d),
            LowerUpper => grapheme_is_lowercase(c) && grapheme_is_uppercase(d),
            DigitUpper => grapheme_is_digit(c) && grapheme_is_uppercase(d),
            UpperDigit => grapheme_is_uppercase(c) && grapheme_is_digit(d),
            DigitLower => grapheme_is_digit(c) && grapheme_is_lowercase(d),
            LowerDigit => grapheme_is_lowercase(c) && grapheme_is_digit(d),
            _ => false,
        }
    }

    fn detect_three(&self, c: &str, d: &str, e: &str) -> bool {
        use Boundary::*;
        if let Acronym = self {
            grapheme_is_uppercase(c)
                && grapheme_is_uppercase(d)
                && grapheme_is_lowercase(e)
        } else {
            false
        }
    }
}

fn grapheme_is_digit(c: &str) -> bool {
    c.chars().all(|c| c.is_ascii_digit())
}

fn grapheme_is_uppercase(c: &str) -> bool {
    c.to_uppercase() != c.to_lowercase() && c == c.to_uppercase()
}

fn grapheme_is_lowercase(c: &str) -> bool {
    c.to_uppercase() != c.to_lowercase() && c == c.to_lowercase()
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
        .graphemes(true)
        .enumerate()
        .filter(|(_, c)| boundaries.iter().any(|b| b.detect_one(*c)))
        .map(|(i, _)| i + 1)
        .collect();

    let words = replace_at_indicies(s, single_splits);

    let final_words = words.iter().flat_map(|&w| {
        let left_iter = w.graphemes(true);
        let mid_iter = w.graphemes(true).skip(1);
        let right_iter = w.graphemes(true).skip(2);

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
    use strum::IntoEnumIterator;

    #[test]
    fn all_boundaries_in_iter() {
        let all = Boundary::all();
        for boundary in Boundary::iter() {
            assert!(all.contains(&boundary));
        }
    }

    #[test]
    fn split_on_delims() {
        assert_eq!(
            vec!["my", "word", "list", "separated", "by", "delims"],
            split("my_word-list separated-by_delims", &Boundary::delims())
        )
    }

    #[test]
    fn boundaries_found_in_string() {
        use Boundary::*;
        assert_eq!(
            vec![LowerUpper, UpperLower, LowerDigit],
            Boundary::list_from("a8.Aa.aA")
        );
        assert_eq!(
            Boundary::digits(),
            Boundary::list_from("b1B1b")
        );
        assert_eq!(
            vec![Hyphen, Underscore, Space, Acronym],
            Boundary::list_from("AAa -_")
        );
    }
}
