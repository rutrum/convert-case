use unicode_segmentation::UnicodeSegmentation;

use alloc::vec::Vec;

/// Per-grapheme classification flags used to accelerate built-in boundary checks.
/// Each grapheme is classified once; then all 9 built-in boundaries check
/// these flags instead of calling `grapheme_is_uppercase` etc. every time.
const UPPER: u8 = 0b00_0001;
const LOWER: u8 = 0b00_0010;
const DIGIT: u8 = 0b00_0100;
const UNDER: u8 = 0b00_1000;
const HYPH: u8 = 0b01_0000;
const SPACE: u8 = 0b10_0000;

#[derive(Clone, Copy, Default)]
struct GraphemeFlags(u8);

impl GraphemeFlags {
    fn classify(grapheme: &str) -> Self {
        if let [b] = grapheme.as_bytes() {
            let mut f = 0u8;
            if b.is_ascii_uppercase() {
                f |= UPPER;
            }
            if b.is_ascii_lowercase() {
                f |= LOWER;
            }
            if b.is_ascii_digit() {
                f |= DIGIT;
            }
            if *b == b'_' {
                f |= UNDER;
            }
            if *b == b'-' {
                f |= HYPH;
            }
            if *b == b' ' {
                f |= SPACE;
            }
            Self(f)
        } else {
            // Multi-byte unicode grapheme: classify by the first codepoint.
            let mut f = 0u8;
            if let Some(ch) = grapheme.chars().next() {
                if ch.is_uppercase() {
                    f |= UPPER;
                }
                if ch.is_lowercase() {
                    f |= LOWER;
                }
                // Digits in non-ASCII scripts still count as letters for boundary
                // purposes, not digits — we only split on ASCII digits.
            }
            Self(f)
        }
    }

    fn is_upper(self) -> bool {
        self.0 & UPPER != 0
    }
    fn is_lower(self) -> bool {
        self.0 & LOWER != 0
    }
    fn is_digit(self) -> bool {
        self.0 & DIGIT != 0
    }
    fn is_under(self) -> bool {
        self.0 & UNDER != 0
    }
    fn is_hyph(self) -> bool {
        self.0 & HYPH != 0
    }
    fn is_space(self) -> bool {
        self.0 & SPACE != 0
    }
}

fn grapheme_is_digit(c: &&str) -> bool {
    c.chars().all(|c| c.is_ascii_digit())
}

fn grapheme_is_uppercase(c: &&str) -> bool {
    // Fast path for single-byte (ASCII) graphemes — O(1), no allocation.
    if let [b] = c.as_bytes() {
        return b.is_ascii_uppercase();
    }
    c.to_uppercase() != c.to_lowercase() && *c == c.to_uppercase()
}

fn grapheme_is_lowercase(c: &&str) -> bool {
    // Fast path for single-byte (ASCII) graphemes — O(1), no allocation.
    if let [b] = c.as_bytes() {
        return b.is_ascii_lowercase();
    }
    c.to_uppercase() != c.to_lowercase() && *c == c.to_lowercase()
}

/// Conditions for splitting an identifier into words.
///
/// Some boundaries, [`Hyphen`](Boundary::Hyphen), [`Underscore`](Boundary::Underscore), and [`Space`](Boundary::Space),
/// consume the character they split on, whereas the other boundaries do not.
///
/// `Boundary` includes methods that return useful groups of boundaries.  It also
/// contains the [`defaults_from`](Boundary::defaults_from) method which will generate a subset
/// of default boundaries based on the boundaries present in a string.
///
/// You can also create custom delimiter boundaries using the [`separator`](crate::separator)
/// macro or directly instantiate `Boundary` for complex boundary conditions.
/// ```
/// use convert_case::{Boundary, Case, Casing, Converter};
///
/// assert_eq!(
///     "TransformationsIn3D"
///         .from_case(Case::Camel)
///         .remove_boundaries(&Boundary::digit_letter())
///         .to_case(Case::Snake),
///     "transformations_in_3d",
/// );
///
/// let conv = Converter::new()
///     .set_boundaries(&Boundary::defaults_from("aA "))
///     .to_case(Case::Title);
/// assert_eq!(conv.convert("myVariable Name"), "My Variable Name");
/// ```
///
/// ## Example
///
/// For more complex boundaries, such as splitting based on the first character being a certain
/// symbol and the second is lowercase, you can instantiate a boundary directly.
///
/// ```
/// # use convert_case::{Boundary, Case, Casing};
/// let at_then_letter = Boundary::Custom {
///     condition: |s| {
///         s.get(0).map(|c| *c == "@") == Some(true)
///             && s.get(1).map(|c| *c == c.to_lowercase()) == Some(true)
///     },
///     start: 1,
///     len: 0,
/// };
/// assert_eq!(
///     "name@domain"
///         .set_boundaries(&[at_then_letter])
///         .to_case(Case::Title),
///     "Name@ Domain",
/// )
/// ```

#[derive(Debug, Clone, Copy)]
pub enum Boundary {
    Custom {
        /// A function that determines if this boundary is present at the start
        /// of the string.  Second argument is the `arg` field.
        condition: fn(&[&str]) -> bool,
        /// Where the beginning of the boundary is.
        start: usize,
        /// The length of the boundary.  This is the number of graphemes that
        /// are removed when splitting.
        len: usize,
    },

    /// Splits on `-`, consuming the character on segmentation.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("-"),
    ///     vec![Boundary::Hyphen],
    /// );
    /// ```
    Hyphen,

    /// Splits on `_`, consuming the character on segmentation.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("_"),
    ///     vec![Boundary::Underscore],
    /// );
    /// ```
    Underscore,

    /// Splits on space, consuming the character on segmentation.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from(" "),
    ///     vec![Boundary::Space],
    /// );
    /// ```
    Space,

    /// Splits where an uppercase letter is followed by a lowercase letter.  This is seldom used,
    /// and is **not** included in the [defaults](Boundary::defaults).
    /// ```
    /// # use convert_case::Boundary;
    /// assert!(Boundary::defaults_from("Aa").is_empty());
    UpperLower,

    /// Splits where a lowercase letter is followed by an uppercase letter.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("aA"),
    ///     vec![Boundary::LowerUpper],
    /// );
    /// ```
    LowerUpper,

    /// Splits where digit is followed by an uppercase letter.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("1A"),
    ///     vec![Boundary::DigitUpper],
    /// );
    /// ```
    DigitUpper,

    /// Splits where an uppercase letter is followed by a digit.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("A1"),
    ///     vec![Boundary::UpperDigit],
    /// );
    /// ```
    UpperDigit,

    /// Splits where digit is followed by a lowercase letter.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("1a"),
    ///     vec![Boundary::DigitLower],
    /// );
    /// ```
    DigitLower,

    /// Splits where a lowercase letter is followed by a digit.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("a1"),
    ///     vec![Boundary::LowerDigit],
    /// );
    /// ```
    LowerDigit,

    /// Acronyms are identified by two uppercase letters followed by a lowercase letter.
    /// The word boundary is between the two uppercase letters.  For example, "HTTPRequest"
    /// would have an acronym boundary identified at "PRe" and split into "HTTP" and "Request".
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("AAa"),
    ///     vec![Boundary::Acronym],
    /// );
    /// ```
    Acronym,
}

impl Boundary {
    pub fn matches(self, s: &[&str]) -> bool {
        use Boundary::*;
        match self {
            Underscore => s.first() == Some(&"_"),
            Hyphen => s.first() == Some(&"-"),
            Space => s.first() == Some(&" "),
            Acronym => {
                s.first().map(grapheme_is_uppercase) == Some(true)
                    && s.get(1).map(grapheme_is_uppercase) == Some(true)
                    && s.get(2).map(grapheme_is_lowercase) == Some(true)
            }
            LowerUpper => {
                s.first().map(grapheme_is_lowercase) == Some(true)
                    && s.get(1).map(grapheme_is_uppercase) == Some(true)
            }
            UpperLower => {
                s.first().map(grapheme_is_uppercase) == Some(true)
                    && s.get(1).map(grapheme_is_lowercase) == Some(true)
            }
            LowerDigit => {
                s.first().map(grapheme_is_lowercase) == Some(true)
                    && s.get(1).map(grapheme_is_digit) == Some(true)
            }
            UpperDigit => {
                s.first().map(grapheme_is_uppercase) == Some(true)
                    && s.get(1).map(grapheme_is_digit) == Some(true)
            }
            DigitLower => {
                s.first().map(grapheme_is_digit) == Some(true)
                    && s.get(1).map(grapheme_is_lowercase) == Some(true)
            }
            DigitUpper => {
                s.first().map(grapheme_is_digit) == Some(true)
                    && s.get(1).map(grapheme_is_uppercase) == Some(true)
            }
            Custom { condition, .. } => condition(s),
        }
    }

    /// The number of graphemes consumed when splitting at the boundary.
    pub fn len(self) -> usize {
        use Boundary::*;
        match self {
            Underscore | Hyphen | Space => 1,
            LowerUpper | UpperLower | LowerDigit | UpperDigit | DigitLower | DigitUpper
            | Acronym => 0,
            Custom { len, .. } => len,
        }
    }

    /// Returns true if this boundary consumes no graphemes when splitting.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// The index of the character to split at.
    pub fn start(self) -> usize {
        use Boundary::*;
        match self {
            Underscore | Hyphen | Space => 0,
            LowerUpper | UpperLower | LowerDigit | UpperDigit | DigitLower | DigitUpper
            | Acronym => 1,
            Custom { start, .. } => start,
        }
    }

    /// The default list of boundaries used when `Casing::to_case` is called directly
    /// and in a `Converter` generated from `Converter::new()`.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults(),
    ///     [
    ///         Boundary::Underscore,
    ///         Boundary::Hyphen,
    ///         Boundary::Space,
    ///         Boundary::LowerUpper,
    ///         Boundary::LowerDigit,
    ///         Boundary::UpperDigit,
    ///         Boundary::DigitLower,
    ///         Boundary::DigitUpper,
    ///         Boundary::Acronym,
    ///     ],
    /// );
    /// ```
    pub const fn defaults() -> [Boundary; 9] {
        [
            Boundary::Underscore,
            Boundary::Hyphen,
            Boundary::Space,
            Boundary::LowerUpper,
            Boundary::LowerDigit,
            Boundary::UpperDigit,
            Boundary::DigitLower,
            Boundary::DigitUpper,
            Boundary::Acronym,
        ]
    }

    /// Returns the boundaries that involve digits.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::digits(),
    ///     [
    ///         Boundary::LowerDigit,
    ///         Boundary::UpperDigit,
    ///         Boundary::DigitLower,
    ///         Boundary::DigitUpper,
    ///     ],
    /// );
    /// ```
    pub const fn digits() -> [Boundary; 4] {
        [
            Boundary::LowerDigit,
            Boundary::UpperDigit,
            Boundary::DigitLower,
            Boundary::DigitUpper,
        ]
    }

    /// Returns the boundaries that are letters followed by digits.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::letter_digit(),
    ///     [
    ///         Boundary::LowerDigit,
    ///         Boundary::UpperDigit,
    ///     ],
    /// );
    /// ```
    pub const fn letter_digit() -> [Boundary; 2] {
        [Boundary::LowerDigit, Boundary::UpperDigit]
    }

    /// Returns the boundaries that are digits followed by letters.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::digit_letter(),
    ///     [
    ///         Boundary::DigitLower,
    ///         Boundary::DigitUpper
    ///     ],
    /// );
    /// ```
    pub const fn digit_letter() -> [Boundary; 2] {
        [Boundary::DigitLower, Boundary::DigitUpper]
    }

    /// Returns a list of all boundaries that are identified within the given string.
    /// Could be a short of writing out all the boundaries in a list directly.  This will not
    /// identify boundary `UpperLower` if it also used as part of `Acronym`.
    ///
    /// If you want to be very explicit and not overlap boundaries, it is recommended to use a colon
    /// character.
    /// ```
    /// # use convert_case::Boundary;
    /// assert_eq!(
    ///     Boundary::defaults_from("aA8a -"),
    ///     vec![
    ///         Boundary::Hyphen,
    ///         Boundary::Space,
    ///         Boundary::LowerUpper,
    ///         Boundary::UpperDigit,
    ///         Boundary::DigitLower,
    ///     ],
    /// );
    /// assert_eq!(
    ///     Boundary::defaults_from("bD:0B:_:AAa"),
    ///     vec![
    ///         Boundary::Underscore,
    ///         Boundary::LowerUpper,
    ///         Boundary::DigitUpper,
    ///         Boundary::Acronym,
    ///     ],
    /// );
    /// ```
    pub fn defaults_from(pattern: &str) -> Vec<Boundary> {
        let mut boundaries = Vec::new();
        for boundary in Boundary::defaults() {
            let parts = split(&pattern, &[boundary]);
            if parts.len() > 1 || parts.is_empty() || parts[0] != pattern {
                boundaries.push(boundary);
            }
        }
        boundaries
    }
}

impl PartialEq for Boundary {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Hyphen, Self::Hyphen) => true,
            (Self::Underscore, Self::Underscore) => true,
            (Self::Space, Self::Space) => true,
            (Self::UpperLower, Self::UpperLower) => true,
            (Self::LowerUpper, Self::LowerUpper) => true,
            (Self::DigitUpper, Self::DigitUpper) => true,
            (Self::UpperDigit, Self::UpperDigit) => true,
            (Self::DigitLower, Self::DigitLower) => true,
            (Self::LowerDigit, Self::LowerDigit) => true,
            (Self::Acronym, Self::Acronym) => true,
            // Custom boundaries are never equal because they contain function pointers,
            // which cannot be reliably compared.
            (Self::Custom { .. }, Self::Custom { .. }) => false,
            _ => false,
        }
    }
}

impl Eq for Boundary {}

impl core::hash::Hash for Boundary {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // Hash only the discriminant. Custom variants can't be meaningfully
        // compared or hashed by their function pointer, so all Custom variants
        // hash to the same value (their discriminant).
        core::mem::discriminant(self).hash(state);
    }
}

/// Split an identifier into a list of words using the list of boundaries.
///
/// This is used internally for splitting an identifier before mutating by
/// a pattern and joining again with a delimiter.
/// ```
/// use convert_case::{Boundary, split};
/// assert_eq!(
///     split(&"one_two-three.four", &[Boundary::Underscore, Boundary::Hyphen]),
///     vec!["one", "two", "three.four"],
/// )
/// ```
pub fn split<'s, T>(s: &'s T, boundaries: &[Boundary]) -> Vec<&'s str>
where
    T: AsRef<str>,
{
    let s = s.as_ref();

    if s.is_empty() {
        return Vec::new();
    }

    let mut words = Vec::new();
    let mut last_boundary_end = 0;

    let (indices, graphemes): (Vec<_>, Vec<_>) = s.grapheme_indices(true).unzip();
    let grapheme_length =
        indices.last().copied().unwrap_or(0) + graphemes.last().map(|g| g.len()).unwrap_or(0);

    // Fast path: when every boundary is a single-character delimiter
    // (underscore, hyphen, space) the check is a direct &str comparison
    // — cheaper than building a classification array.
    let all_simple_delimiters = boundaries
        .iter()
        .all(|b| matches!(b, Boundary::Underscore | Boundary::Hyphen | Boundary::Space));

    if all_simple_delimiters {
        for (i, grapheme) in graphemes.iter().enumerate() {
            for boundary in boundaries {
                let matched = match boundary {
                    Boundary::Underscore => *grapheme == "_",
                    Boundary::Hyphen => *grapheme == "-",
                    Boundary::Space => *grapheme == " ",
                    _ => unreachable!(),
                };

                if matched {
                    let boundary_byte_start: usize = *indices
                        .get(i + boundary.start())
                        .unwrap_or(&grapheme_length);
                    let boundary_byte_end: usize = *indices
                        .get(i + boundary.start() + boundary.len())
                        .unwrap_or(&grapheme_length);
                    words.push(&s[last_boundary_end..boundary_byte_start]);
                    last_boundary_end = boundary_byte_end;
                    break;
                }
            }
        }
        words.push(&s[last_boundary_end..]);
        return words.into_iter().collect();
    }

    // General path: precompute grapheme flags once, then check all
    // boundaries (built-in and custom) against the cache.
    let flags: Vec<GraphemeFlags> = graphemes
        .iter()
        .map(|g| GraphemeFlags::classify(g))
        .collect();

    for (i, _grapheme) in graphemes.iter().enumerate() {
        for boundary in boundaries {
            let matched = match boundary {
                Boundary::Underscore => flags[i].is_under(),
                Boundary::Hyphen => flags[i].is_hyph(),
                Boundary::Space => flags[i].is_space(),
                Boundary::LowerUpper => {
                    i + 1 < graphemes.len() && flags[i].is_lower() && flags[i + 1].is_upper()
                }
                Boundary::UpperLower => {
                    i + 1 < graphemes.len() && flags[i].is_upper() && flags[i + 1].is_lower()
                }
                Boundary::LowerDigit => {
                    i + 1 < graphemes.len() && flags[i].is_lower() && flags[i + 1].is_digit()
                }
                Boundary::UpperDigit => {
                    i + 1 < graphemes.len() && flags[i].is_upper() && flags[i + 1].is_digit()
                }
                Boundary::DigitLower => {
                    i + 1 < graphemes.len() && flags[i].is_digit() && flags[i + 1].is_lower()
                }
                Boundary::DigitUpper => {
                    i + 1 < graphemes.len() && flags[i].is_digit() && flags[i + 1].is_upper()
                }
                Boundary::Acronym => {
                    i + 2 < graphemes.len()
                        && flags[i].is_upper()
                        && flags[i + 1].is_upper()
                        && flags[i + 2].is_lower()
                }
                Boundary::Custom { condition, .. } => condition(&graphemes[i..]),
            };

            if matched {
                let boundary_byte_start: usize = *indices
                    .get(i + boundary.start())
                    .unwrap_or(&grapheme_length);
                let boundary_byte_end: usize = *indices
                    .get(i + boundary.start() + boundary.len())
                    .unwrap_or(&grapheme_length);

                words.push(&s[last_boundary_end..boundary_byte_start]);
                last_boundary_end = boundary_byte_end;
                break;
            }
        }
    }
    words.push(&s[last_boundary_end..]);
    words.into_iter().collect()
}

/// Create a new boundary based on a string.
///
/// This is shorthand for creating a boundary that splits on a specific string, and
/// omits that string from the list of words.  For more information, see [`Boundary`].
/// ```
/// # use convert_case::{Case, Converter, separator};
/// let conv = Converter::new()
///     .set_boundaries(&[separator!("::")])
///     .to_case(Case::Camel);
///
/// assert_eq!(
///     conv.convert("my::var::name"),
///     "myVarName",
/// )
/// ```
#[macro_export]
macro_rules! separator {
    ($delim:expr) => {
        convert_case::Boundary::Custom {
            condition: |s| s.join("").starts_with($delim),
            start: 0,
            len: $delim.len(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use rstest::rstest;

    #[test]
    fn custom_boundary_inequality() {
        // Custom boundaries are never equal because they contain function pointers
        let a = Boundary::Custom {
            condition: |_| true,
            start: 0,
            len: 0,
        };
        let b = a;

        assert_ne!(a, b)
    }

    #[test]
    fn default_boundary_equality() {
        assert_eq!(Boundary::Hyphen, Boundary::Hyphen);
        assert_eq!(Boundary::Space, Boundary::Space);
        assert_ne!(Boundary::Hyphen, Boundary::Space);
    }

    #[rstest]
    #[case(Boundary::Hyphen, "a-b-c", vec!["a", "b", "c"])]
    #[case(Boundary::Underscore, "a_b_c", vec!["a", "b", "c"])]
    #[case(Boundary::Space, "a b c", vec!["a", "b", "c"])]
    #[case(Boundary::LowerUpper, "lowerUpperUpper", vec!["lower", "Upper", "Upper"])]
    #[case(Boundary::UpperLower, "ABc", vec!["AB", "c"])]
    #[case(Boundary::Acronym, "XMLRequest", vec!["XML", "Request"])]
    #[case(Boundary::LowerDigit, "abc123", vec!["abc", "123"])]
    #[case(Boundary::UpperDigit, "ABC123", vec!["ABC", "123"])]
    #[case(Boundary::DigitLower, "123abc", vec!["123", "abc"])]
    #[case(Boundary::DigitUpper, "123ABC", vec!["123", "ABC"])]
    fn split_on_boundary(
        #[case] boundary: Boundary,
        #[case] input: &str,
        #[case] expected: Vec<&str>,
    ) {
        assert_eq!(split(&input, &[boundary]), expected);
    }

    #[test]
    fn split_on_multiple_delimiters() {
        let s = "aaa-bbb_ccc ddd ddd-eee";
        let v = split(
            &s,
            &[Boundary::Space, Boundary::Underscore, Boundary::Hyphen],
        );
        assert_eq!(v, vec!["aaa", "bbb", "ccc", "ddd", "ddd", "eee"]);
    }

    #[test]
    fn boundaries_found_in_string() {
        // upper lower is no longer a default
        assert_eq!(Boundary::defaults_from(".Aaaa"), Vec::<Boundary>::new());
        assert_eq!(
            Boundary::defaults_from("a8.Aa.aA"),
            vec![Boundary::LowerUpper, Boundary::LowerDigit]
        );
        assert_eq!(
            Boundary::defaults_from("b1B1b"),
            Boundary::digits().to_vec()
        );
        assert_eq!(
            Boundary::defaults_from("AAa -_"),
            vec![
                Boundary::Underscore,
                Boundary::Hyphen,
                Boundary::Space,
                Boundary::Acronym,
            ]
        );
    }
}
