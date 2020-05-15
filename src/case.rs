use crate::Casing;
use std::convert::TryFrom;

#[cfg(test)]
use strum_macros::EnumIter;

/// Defines the type of casing a string can be.
///
/// ```
/// use convert_case::{Case, Casing};
///
/// let super_mario_title: String = "super_mario_64".to_case(Case::Title);
/// assert_eq!("Super Mario 64", super_mario_title);
/// ```
///
/// You can also create a case from a string slice.
/// ```
/// use convert_case::{Case, Casing};
/// use std::convert::TryFrom;
///
/// let snake_case: Case = Case::try_from("snake").unwrap();
/// assert_eq!(Case::Snake, snake_case);
/// ```
///
#[cfg_attr(test, derive(EnumIter))]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Case {
    /// Uppercase strings are delimited by spaces and all characters are uppercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MY VARIABLE NAME", "My variable NAME".to_case(Case::Upper))
    /// ```
    Upper,

    /// Lowercase strings are delimited by spaces and all characters are lowercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("my variable name", "My variable NAME".to_case(Case::Lower))
    /// ```
    Lower,

    /// Title case strings are delimited by spaces. Only the leading character of
    /// each word is uppercase.  No inferences are made about language, so words
    /// like "as", "to", and "for" will still be capitalized.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My Variable Name", "My variable NAME".to_case(Case::Title))
    /// ```
    Title,

    /// Toggle case strings are delimited by spaces.  All characters are uppercase except
    /// for the leading character of each word, which is lowercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("mY vARIABLE nAME", "My variable NAME".to_case(Case::Toggle))
    /// ```
    Toggle,

    /// Camel case strings are lowercase, but for every word _except the first_ the
    /// first letter is capitalized.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("myVariableName", "My variable NAME".to_case(Case::Camel))
    /// ```
    Camel,

    /// Pascal case strings are lowercase, but for every word the
    /// first letter is capitalized.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MyVariableName", "My variable NAME".to_case(Case::Pascal))
    /// ```
    Pascal,
    /// Upper camel case is an alternative name for Pascal case.
    UpperCamel,

    /// Snake case strings are delimited by underscores `_` and are all lowercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("my_variable_name", "My variable NAME".to_case(Case::Snake))
    /// ```
    Snake,

    /// Upper snake case strings are delimited by underscores `_` and are all uppercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MY_VARIABLE_NAME", "My variable NAME".to_case(Case::UpperSnake))
    /// ```
    UpperSnake,
    /// Screaming snake case is an alternative name for upper snake case.
    ScreamingSnake,

    /// Kebab case strings are delimited by hyphens `-` and are all lowercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("my-variable-name", "My variable NAME".to_case(Case::Kebab))
    /// ```
    Kebab,

    /// Cobol case strings are delimited by hyphens `-` and are all uppercase.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MY-VARIABLE-NAME", "My variable NAME".to_case(Case::Cobol))
    /// ```
    Cobol,

    /// Train case strings are delimited by hyphens `-`.  All characters are lowercase
    /// except for the leading character of each word.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("My-Variable-Name", "My variable NAME".to_case(Case::Train))
    /// ```
    Train,

    /// Flat case strings are all lowercase, with no delimiter.  Converting to
    /// this case is **lossy**.  That is, word boundaries are lost.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("myvariablename", "My variable NAME".to_case(Case::Flat))
    /// ```
    Flat,

    /// Upper flat case strings are all uppercase, with no delimiter.  Converting to
    /// this case is **lossy**.  That is, word boundaries are lost.
    ///
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("MYVARIABLENAME", "My variable NAME".to_case(Case::UpperFlat))
    /// ```
    UpperFlat,

    /// Alternating case strings are delimited by spaces.  Characters alternate between uppercase
    /// and lowercase.
    /// ```
    /// use convert_case::{Case, Casing};
    /// assert_eq!("mY vArIaBlE nAmE", "My variable NAME".to_case(Case::Alternating));
    /// ```
    Alternating,
}

impl Case {
    /// Prints the name of the case in that case.
    /// ```
    /// use convert_case::Case;
    ///
    /// assert_eq!("UpperCamelCase", Case::UpperCamel.name_in_case());
    /// assert_eq!("snake_case", Case::Snake.name_in_case());
    /// assert_eq!("Title Case", Case::Title.name_in_case());
    /// ```
    pub fn name_in_case(self) -> String {
        let case_str = format!("{:?}Case", self);
        case_str.to_case(self)
    }

    // Created to avoid using the EnumIter trait from strum in
    // final library.  A test confirms that all cases are listed here.
    /// Returns a vector with all case enum variants.  This was
    /// created for use in the `ccase` binary.
    pub fn all_cases() -> Vec<Case> {
        use Case::*;
        vec![
            Upper,
            Lower,
            Title,
            Toggle,
            Camel,
            Pascal,
            UpperCamel,
            Snake,
            UpperSnake,
            ScreamingSnake,
            Kebab,
            Cobol,
            Train,
            Flat,
            UpperFlat,
            Alternating,
        ]
    }
}

impl TryFrom<&str> for Case {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let case_str = s.to_case(Case::Flat);
        for case in Case::all_cases() {
            if case_str == format!("{:?}", case).to_case(Case::Flat) {
                return Ok(case);
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn all_cases_in_iter() {
        let all = Case::all_cases();
        for case in Case::iter() {
            assert!(all.contains(&case));
        }
    }
}
