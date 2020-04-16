//! Converts to and from various cases.
//!
//! Provides a [`Case`](enum.Case.html) enum which defines a variety of cases to convert into.
//! A `Case` can be used with an item that implements the [`Casing`](trait.Casing.html) trait,
//! which allows the item to be converted to a given case.
//! 
//! You can convert a string slice into a case using the `to_case` method on string slices
//! or on owned strings.
//! ```
//! use convert_case::{Case, Casing};
//! 
//! assert_eq!("Ronnie James Dio", "ronnie james dio".to_case(Case::Title));
//! assert_eq!("ronnieJamesDio", "Ronnie_James_dio".to_case(Case::Camel));
//! assert_eq!("Ronnie-James-Dio", "RONNIE_JAMES_DIO".to_case(Case::Train));
//! ```
//! 
//! You can also explicitly write what case to parse the input as, using `from_case`. 
//! This is useful if the string has a case that is ambiguous.
//! ```
//! use convert_case::{Case, Casing};
//! 
//! let filename = "2020-04-16_my_cat_cali".from_case(Case::Snake).to_case(Case::Title);
//! assert_eq!("2020-04-16 My Cat Cali", filename);
//! ```
//! 
//! # Note on Accuracy
//! 
//! The `Casing` methods `from_case` and `to_case` do not fail.  Conversion to a case will always
//! succeed.  However, the results can still be unexpected.  
//!
//! The `to_case` method uses a series checks to determine where to split 
//! the string into words.  Even if that method is explicit
//! from the `from_case` method, it can always parse the entire string as a single word.
//! 
//! These examples demonstrate some unexpected behavior.
//! ```
//! use convert_case::{Case, Casing};
//!
//! // Mistakenly parsing using Case::Snake
//! assert_eq!("My-kebab-var", "my-kebab-var".from_case(Case::Snake).to_case(Case::Title));
//!
//! // Converts using an unexpected method
//! assert_eq!("Mymany Casevariable", "myMany-caseVariable".to_case(Case::Title));
//! ```
//! 
//! If your string has a variety of cases, try splitting across some delimiter before using
//! the `from_case` and `to_case` methods.

mod case;
mod words;
pub use case::Case;
use words::Words;

/// Describes items that can be converted into a case.
///
/// Currently implemented for string slices `&str` and owned strings `String`.
pub trait Casing {
    /// References `self` and converts to the given case.
    fn to_case(&self, case: Case) -> String;

    /// Creates a `FromCasing` struct, which saves information about
    /// how to parse `self` before converting to a case.
    fn from_case(&self, case: Case) -> FromCasing;
}

impl Casing for str {
    fn to_case(&self, case: Case) -> String {
        Words::new(self).into_case(case)
    }

    fn from_case(&self, case: Case) -> FromCasing {
        FromCasing::new(self.to_string(), case)
    }
}

impl Casing for String {
    fn to_case(&self, case: Case) -> String {
        Words::new(self).into_case(case)
    }

    fn from_case(&self, case: Case) -> FromCasing {
        FromCasing::new(self.to_string(), case)
    }
}

/// Holds information about parsing before converting into a case.
///
/// This struct is used when invoking the `from_case` method on
/// `Casing`.  `FromCasing` also implements `Casing`.
/// ```
/// use convert_case::{Case, Casing};
///
/// let title = "ninety-nine_problems".from_case(Case::Snake).to_case(Case::Title);
/// assert_eq!("Ninety-nine Problems", title);
/// ```
pub struct FromCasing {
    name: String,
    case: Case,
}

impl FromCasing {
    fn new(name: String, case: Case) -> Self {
        Self { name, case }
    }
}

impl Casing for FromCasing {
    fn to_case(&self, case: Case) -> String {
        Words::from_casing(&self.name, self.case).into_case(case)
    }

    fn from_case(&self, case: Case) -> Self {
        Self::new(self.name.to_string(), case)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lossless_against_lossless() {
        let examples = vec![
            (Case::Lower, "my variable 2 name"),
            (Case::Upper, "MY VARIABLE 2 NAME"),
            (Case::Title, "My Variable 2 Name"),
            (Case::Camel, "myVariable2Name"),
            (Case::Pascal, "MyVariable2Name"),
            (Case::Snake, "my_variable_2_name"),
            (Case::ScreamingSnake, "MY_VARIABLE_2_NAME"),
            (Case::Kebab, "my-variable-2-name"),
            (Case::Cobol, "MY-VARIABLE-2-NAME"),
            (Case::Toggle, "mY vARIABLE 2 nAME"),
            (Case::Train, "My-Variable-2-Name"),
        ];

        for (case_a, str_a) in examples.iter() {
            for (case_b, str_b) in examples.iter() {
                assert_eq!(*str_a, str_b.from_case(*case_b).to_case(*case_a))
            }
        }
    }

    #[test]
    fn empty_string() {
        assert_eq!("", "".to_case(Case::Upper));
    }

    #[test]
    fn owned_string() {
        assert_eq!("test_variable", String::from("TestVariable").to_case(Case::Snake))
    }
}
