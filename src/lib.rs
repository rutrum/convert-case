//! Converts to and from various cases.
//!
//! # Command Line Utility `ccase`
//!
//! Since version "0.3.0" this crate is just a case conversion _library_.  The command line utility
//! that uses the tools in this library has been moved to the `ccase` crate.  You can read about it
//! at the [GitHub repository](https://github.com/rutrum/convert-case/tree/master/ccase).
//!
//! # Rust Library
//!
//! Provides a [`Case`](enum.Case.html) enum which defines a variety of cases to convert into.
//! A `Case` can be used with an item that implements the [`Casing`](trait.Casing.html) trait,
//! which allows the item to be converted to a given case.
//!
//! You can convert a string or string slice into a case using the `to_case` method.
//! ```
//! use convert_case::{Case, Casing};
//!
//! assert_eq!("Ronnie James Dio", "ronnie james dio".to_case(Case::Title));
//! assert_eq!("ronnieJamesDio", "Ronnie_James_dio".to_case(Case::Camel));
//! assert_eq!("Ronnie-James-Dio", "RONNIE_JAMES_DIO".to_case(Case::Train));
//! ```
//!
//! By default, `to_case` will split along all word boundaries, that is
//! * space characters ` `,
//! * underscores `_`,
//! * hyphens `-`,
//! * changes in capitalization from lowercase to uppercase `aA`,
//! * and adjacent digits and letters `a1` and `1a`.
//!
//! For more accuracy, the `from_case` method splits based on the word boundaries
//! of a particular case.  For example, splitting from snake case will only treat
//! underscores as word boundaries.
//! ```
//! use convert_case::{Case, Casing};
//!
//! assert_eq!(
//!     "2020 04 16 My Cat Cali",
//!     "2020-04-16_my_cat_cali".to_case(Case::Title)
//! );
//! assert_eq!(
//!     "2020-04-16 My Cat Cali",
//!     "2020-04-16_my_cat_cali".from_case(Case::Snake).to_case(Case::Title)
//! );
//! ```
//!
//! By default (and when converting from camel case or similar cases) `convert_case`
//! will detect acronyms.  It also ignores any leading, trailing, or duplicate delimiters.
//! ```
//! use convert_case::{Case, Casing};
//!
//! assert_eq!("io_stream", "IOStream".to_case(Case::Snake));
//! assert_eq!("my_json_parser", "myJSONParser".to_case(Case::Snake));
//!
//! assert_eq!("weird_var_name", "__weird--var _name-".to_case(Case::Snake));
//! ```
//!
//! It also works non-ascii characters.  However, no inferences on the language itself is made.
//! For instance, the digraph `ij` in Dutch will not be capitalized, because it is represented
//! as two distinct Unicode characters.  However, `æ` would be capitalized.
//! ```
//! use convert_case::{Case, Casing};
//!
//! assert_eq!("granat-äpfel", "GranatÄpfel".to_case(Case::Kebab));
//!
//! // The example from str::to_lowercase documentation
//! let odysseus = "ὈΔΥΣΣΕΎΣ";
//! assert_eq!("ὀδυσσεύς", odysseus.to_case(Case::Lower));
//! ```
//!
//! For the purposes of case conversion, characters followed by digits and vice-versa are
//! considered word boundaries.  In addition, any special ASCII characters (besides `_` and `-`)
//! are ignored.
//! ```
//! use convert_case::{Case, Casing};
//!
//! assert_eq!("e_5150", "E5150".to_case(Case::Snake));
//! assert_eq!("10,000_days", "10,000Days".to_case(Case::Snake));
//! assert_eq!("HELLO, WORLD!", "Hello, world!".to_case(Case::Upper));
//! assert_eq!("One\ntwo\nthree", "ONE\nTWO\nTHREE".to_case(Case::Title));
//! ```
//!
//! # Note on Accuracy
//!
//! The `Casing` methods `from_case` and `to_case` do not fail.  Conversion to a case will always
//! succeed.  However, the results can still be unexpected.  Failure to detect any word boundaries
//! for a particular case means the entire string will be considered a single word.
//! ```
//! use convert_case::{Case, Casing};
//!
//! // Mistakenly parsing using Case::Snake
//! assert_eq!("My-kebab-var", "my-kebab-var".from_case(Case::Snake).to_case(Case::Title));
//!
//! // Converts using an unexpected method
//! assert_eq!("my_kebab_like_variable", "myKebab-like-variable".to_case(Case::Snake));
//! ```
//!
//! # Random Feature
//!
//! To ensure this library had zero dependencies, randomness was moved to the _random_ feature,
//! which requires the `rand` crate. You can enable this feature by including the
//! following in your `Cargo.toml`.
//! ```{toml}
//! [dependencies]
//! convert_case = { version = "^0.3.0", features = ["random"] }
//! ```
//! This will add two additional cases: Random and PseudoRandom.  You can read about their
//! construction in the [Case enum](enum.Case.html).

mod case;
mod converter;
mod pattern;
mod segmentation;

pub use case::Case;
pub use converter::Converter;
pub use pattern::Pattern;
pub use segmentation::Boundary;

/// Describes items that can be converted into a case.
///
/// Implemented for string slices `&str`, `String`, and `&String`.
pub trait Casing<T: AsRef<str>> {
    /// References `self` and converts to the given case.
    fn to_case(&self, case: Case) -> String;

    /// Creates a `Converter` struct, which saves information about
    /// how to parse `self` before converting to a case.
    fn from_case(&self, case: Case) -> StateConverter<T>;

    /// Creates a write docs
    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<T>;

    /// Determines if `self` is of the given case.
    fn is_case(&self, case: Case) -> bool;
}

impl<T: AsRef<str>> Casing<T> for T
where
    String: PartialEq<T>,
{
    fn to_case(&self, case: Case) -> String {
        StateConverter::new(self).to_case(case)
    }

    fn with_boundaries(&self, bs: &[Boundary]) -> StateConverter<T> {
        StateConverter::new(self).with_boundaries(bs)
    }

    fn from_case(&self, case: Case) -> StateConverter<T> {
        StateConverter::new_from_case(self, case)
    }

    fn is_case(&self, case: Case) -> bool {
        &self.to_case(case) == self
    }
}

/// Holds information about parsing before converting into a case.
///
/// This struct is used when invoking the `from_case` method on
/// `Casing`.
/// ```
/// use convert_case::{Case, Casing};
///
/// let title = "ninety-nine_problems".from_case(Case::Snake).to_case(Case::Title);
/// assert_eq!("Ninety-nine Problems", title);
/// ```
pub struct StateConverter<'a, T: AsRef<str>> {
    s: &'a T,
    conv: Converter,
}

impl<'a, T: AsRef<str>> StateConverter<'a, T> {
    /// Only called by Casing function to_case()
    fn new(s: &'a T) -> Self {
        Self {
            s,
            conv: Converter::new(),
        }
    }

    /// Only called by Casing function from_case()
    fn new_from_case(s: &'a T, case: Case) -> Self {
        Self {
            s,
            conv: Converter::new().from_case(case),
        }
    }

    pub fn from_case(self, case: Case) -> Self {
        Self {
            conv: self.conv.from_case(case),
            ..self
        }
    }

    pub fn with_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.set_boundaries(bs),
        }
    }

    pub fn without_boundaries(self, bs: &[Boundary]) -> Self {
        Self {
            s: self.s,
            conv: self.conv.remove_boundaries(bs),
        }
    }

    /// Consumes the `StateConverter` and converts the string.
    pub fn convert(self) -> String {
        self.conv.convert(self.s)
    }

    pub fn to_case(self, case: Case) -> String {
        self.conv.to_case(case).convert(self.s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strum::IntoEnumIterator;

    fn possible_cases(s: &str) -> Vec<Case> {
        Case::deterministic_cases()
            .into_iter()
            .filter(|case| s.from_case(*case).to_case(*case) == s)
            .collect()
    }

    #[test]
    fn lossless_against_lossless() {
        let examples = vec![
            (Case::Lower, "my variable 22 name"),
            (Case::Upper, "MY VARIABLE 22 NAME"),
            (Case::Title, "My Variable 22 Name"),
            (Case::Camel, "myVariable22Name"),
            (Case::Pascal, "MyVariable22Name"),
            (Case::Snake, "my_variable_22_name"),
            (Case::UpperSnake, "MY_VARIABLE_22_NAME"),
            (Case::Kebab, "my-variable-22-name"),
            (Case::Cobol, "MY-VARIABLE-22-NAME"),
            (Case::Toggle, "mY vARIABLE 22 nAME"),
            (Case::Train, "My-Variable-22-Name"),
            (Case::Alternating, "mY vArIaBlE 22 nAmE"),
        ];

        for (case_a, str_a) in examples.iter() {
            for (case_b, str_b) in examples.iter() {
                assert_eq!(*str_a, str_b.from_case(*case_b).to_case(*case_a))
            }
        }
    }

    #[test]
    fn obvious_default_parsing() {
        let examples = vec![
            "SuperMario64Game",
            "super-mario64-game",
            "superMario64 game",
            "Super Mario 64_game",
            "SUPERMario 64-game",
            "super_mario-64 game",
        ];

        for example in examples {
            assert_eq!("super_mario_64_game", example.to_case(Case::Snake));
        }
    }

    #[test]
    fn multiline_strings() {
        assert_eq!("One\ntwo\nthree", "one\ntwo\nthree".to_case(Case::Title));
    }

    #[test]
    fn camel_case_acroynms() {
        assert_eq!(
            "xml_http_request",
            "XMLHttpRequest".from_case(Case::Camel).to_case(Case::Snake)
        );
        assert_eq!(
            "xml_http_request",
            "XMLHttpRequest"
                .from_case(Case::UpperCamel)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "xml_http_request",
            "XMLHttpRequest"
                .from_case(Case::Pascal)
                .to_case(Case::Snake)
        );
    }

    #[test]
    fn leading_tailing_delimeters() {
        assert_eq!(
            "leading_underscore",
            "_leading_underscore"
                .from_case(Case::Snake)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "tailing_underscore",
            "tailing_underscore_"
                .from_case(Case::Snake)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "leading_hyphen",
            "-leading-hyphen"
                .from_case(Case::Kebab)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "tailing_hyphen",
            "tailing-hyphen-"
                .from_case(Case::Kebab)
                .to_case(Case::Snake)
        );
    }

    #[test]
    fn double_delimeters() {
        assert_eq!(
            "many_underscores",
            "many___underscores"
                .from_case(Case::Snake)
                .to_case(Case::Snake)
        );
        assert_eq!(
            "many-underscores",
            "many---underscores"
                .from_case(Case::Kebab)
                .to_case(Case::Kebab)
        );
    }

    #[test]
    fn early_word_boundaries() {
        assert_eq!(
            "a_bagel",
            "aBagel".from_case(Case::Camel).to_case(Case::Snake)
        );
    }

    #[test]
    fn late_word_boundaries() {
        assert_eq!(
            "team_a",
            "teamA".from_case(Case::Camel).to_case(Case::Snake)
        );
    }

    #[test]
    fn empty_string() {
        for (case_a, case_b) in Case::iter().zip(Case::iter()) {
            assert_eq!("", "".from_case(case_a).to_case(case_b));
        }
    }

    #[test]
    fn owned_string() {
        assert_eq!(
            "test_variable",
            String::from("TestVariable").to_case(Case::Snake)
        )
    }

    #[test]
    fn default_all_boundaries() {
        assert_eq!(
            "abc_abc_abc_abc_abc_abc",
            "ABC-abc_abcAbc ABCAbc".to_case(Case::Snake)
        );
    }

    #[test]
    fn alternating_ignore_symbols() {
        assert_eq!("tHaT's", "that's".to_case(Case::Alternating));
    }

    #[test]
    fn string_is_snake() {
        assert!("im_snake_case".is_case(Case::Snake));
        assert!(!"im_NOTsnake_case".is_case(Case::Snake));
    }

    #[test]
    fn string_is_kebab() {
        assert!("im-kebab-case".is_case(Case::Kebab));
        assert!(!"im_not_kebab".is_case(Case::Kebab));
    }

    #[test]
    fn remove_boundaries() {
        assert_eq!(
            "m02_s05_binary_trees.pdf",
            "M02S05BinaryTrees.pdf"
                .from_case(Case::Pascal)
                .without_boundaries(&[Boundary::UpperDigit])
                .to_case(Case::Snake)
        );
    }

    #[test]
    fn with_boundaries() {
        assert_eq!(
            "my-dumb-file-name",
            "my_dumbFileName"
                .with_boundaries(&[Boundary::Underscore, Boundary::LowerUpper])
                .to_case(Case::Kebab)
        );
    }

    #[cfg(feature = "random")]
    #[test]
    fn random_case_boundaries() {
        for random_case in Case::random_cases() {
            assert_eq!(
                "split_by_spaces",
                "Split By Spaces"
                    .from_case(random_case)
                    .to_case(Case::Snake)
            );
        }
    }

    #[test]
    fn early_convert() {
        assert_eq!(
            "myCrazywordLIST",
            "myCrazy-word-LIST".from_case(Case::Kebab).convert(),
        )
    }

    #[test]
    fn multiple_from_case() {
        assert_eq!(
            "longtime_nosee",
            "LongTime NoSee"
                .from_case(Case::Camel)
                .from_case(Case::Title)
                .to_case(Case::Snake),
        )
    }

    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn detect_many_cases() {
        let lower_cases_vec = possible_cases(&"asef");
        let lower_cases_set = HashSet::from_iter(lower_cases_vec.into_iter());
        let mut actual = HashSet::new();
        actual.insert(Case::Lower);
        actual.insert(Case::Camel);
        actual.insert(Case::Snake);
        actual.insert(Case::Kebab);
        actual.insert(Case::Flat);
        assert_eq!(lower_cases_set, actual);

        let lower_cases_vec = possible_cases(&"asefCase");
        let lower_cases_set = HashSet::from_iter(lower_cases_vec.into_iter());
        let mut actual = HashSet::new();
        actual.insert(Case::Camel);
        assert_eq!(lower_cases_set, actual);
    }

    #[test]
    fn detect_each_case() {
        let s = "My String Identifier".to_string();
        for case in Case::deterministic_cases() {
            let new_s = s.from_case(case).to_case(case);
            let possible = possible_cases(&new_s);
            println!("{} {:?} {:?}", new_s, case, possible);
            assert!(possible.iter().any(|c| c == &case));
        }
    }
}
