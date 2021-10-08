use crate::Case;
use crate::boundary;

#[cfg(feature = "random")]
use rand::prelude::*;

pub(super) struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn new(name: &str) -> Self {
        use boundary::Boundary::*;
        let default_boundaries = vec![
            Underscore, Hyphen, Space,
            LowerUpper, UpperDigit, DigitUpper,
            DigitLower, LowerDigit, Acronyms,
        ];

        let words = boundary::split(name, &default_boundaries);

        Self { words }
    }

    pub fn from_casing(name: &str, case: Case) -> Self {
        let bs = case.boundaries();

        let words = boundary::split(name, &bs);

        Self { words }
    }

    pub fn into_case(self, case: Case) -> String {
        let words = self.words;
        let pattern = case.pattern();
        let delim = case.delim();
        pattern.mutate(&words).join(delim)
    }
}
