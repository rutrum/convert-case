use crate::boundary;
use crate::Boundary;
use crate::Pattern;
use crate::Case;

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
    boundaries: Vec<Boundary>,
    pattern: Option<Pattern>,
    delim: String,
    // delete the 3 above
    
    // conv: Converter
}

impl<'a, T: AsRef<str>> StateConverter<'a, T> {
    pub(crate) fn new(s: &'a T) -> Self {
        Self {
            s,
            boundaries: Boundary::defaults(),
            delim: String::new(),
            pattern: None,
        }
    }

    pub(crate) fn new_from_case(s: &'a T, case: Case) -> Self {
        Self {
            s,
            boundaries: case.boundaries(),
            delim: String::new(),
            pattern: None, // doesn't matter
        }
    }

    pub fn convert(self) -> String {
        let words = boundary::split(&self.s, &self.boundaries);
        if let Some(p) = self.pattern {
            p.mutate(&words).join(&self.delim)
        } else {
            words.join(&self.delim)
        }
    }

    pub fn to_case(mut self, case: Case) -> String {
        self.pattern = Some(case.pattern());
        self.delim = case.delim().to_string();
        self.convert()
    }

    pub fn from_case(&mut self, case: Case) {
        self.boundaries = case.boundaries();
    }
}


/// Do something like this
pub struct Converter {
    boundaries: Vec<Boundary>,
    pattern: Option<Pattern>,
    delim: String,
}

impl Default for Converter {
    fn default() -> Self {
        Converter {
            boundaries: Boundary::defaults(),
            pattern: None,
            delim: String::new(),
        }
    }
}

impl Converter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn convert<T>(&self, s: T) -> String where T: AsRef<str> {
        let words = boundary::split(&s, &self.boundaries);
        if let Some(p) = self.pattern {
            p.mutate(&words).join(&self.delim)
        } else {
            words.join(&self.delim)
        }
    }

    pub fn to_case(mut self, case: Case) -> Self {
        self.pattern = Some(case.pattern());
        self.delim = case.delim().to_string();
        self
    }

    pub fn from_case(mut self, case: Case) -> Self {
        self.boundaries = case.boundaries();
        self
    }

    pub fn add_boundary(mut self, b: &Boundary) -> Self {
        self.boundaries.push(*b);
        self
    }

    pub fn add_boundaries(mut self, bs: &Vec<Boundary>) -> Self {
        self.boundaries.extend(bs);
        self
    }

    pub fn set_boundaries(mut self, bs: &Vec<Boundary>) -> Self {
        self.boundaries = bs.clone();
        self
    }

    pub fn set_delim<T>(mut self, d: T) -> Self where T: ToString {
        self.delim = d.to_string();
        self
    }

    pub fn remove_delim(mut self) -> Self {
        self.delim = String::new();
        self
    }

    pub fn set_pattern(mut self, p: Pattern) -> Self {
        self.pattern = Some(p);
        self
    }

    pub fn remove_pattern(mut self) -> Self {
        self.pattern = None;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Casing;
    use crate::Pattern;

    #[test]
    fn snake_converter_from_case() {
        let conv = Converter::new().to_case(Case::Snake);
        let s = String::from("my var name");
        assert_eq!(s.to_case(Case::Snake), conv.convert(s));
    }

    #[test]
    fn snake_converter_from_scratch() {
        let conv = Converter::new()
            .set_delim("_")
            .set_pattern(Pattern::Lowercase);
        let s = String::from("my var name");
        assert_eq!(s.to_case(Case::Snake), conv.convert(s));
    }

    #[test]
    fn custom_pattern() {
        let conv = Converter::new()
            .to_case(Case::Snake)
            .set_pattern(Pattern::Sentence);
        assert_eq!("Bjarne_case", conv.convert("bjarne case"));
    }
}
