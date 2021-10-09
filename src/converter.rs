use crate::boundary;
use crate::Boundary;
use crate::Pattern;
use crate::Case;

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

    pub fn add_boundary(mut self, b: Boundary) -> Self {
        self.boundaries.push(b);
        self
    }

    pub fn add_boundaries(mut self, bs: &[Boundary]) -> Self {
        self.boundaries.extend(bs);
        self
    }

    pub fn set_boundaries(mut self, bs: &[Boundary]) -> Self {
        self.boundaries = bs.to_vec();
        self
    }

    pub fn remove_boundary(mut self, b: Boundary) -> Self {
        self.boundaries.retain(|&x| x != b);
        self
    }

    pub fn remove_boundaries(mut self, bs: &[Boundary]) -> Self {
        for b in bs {
            self.boundaries.retain(|&x| x != *b);
        }
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

    #[test]
    fn custom_delim() {
        let conv = Converter::new()
            .set_delim("..");
        assert_eq!("oh..My", conv.convert("ohMy"));
    }

    #[test]
    fn no_pattern() {
        let conv = Converter::new()
            .from_case(Case::Title)
            .to_case(Case::Kebab)
            .remove_pattern();
        assert_eq!("wIErd-CASing", conv.convert("wIErd CASing"));
    }

    #[test]
    fn no_delim() {
        let conv = Converter::new()
            .from_case(Case::Title)
            .to_case(Case::Kebab)
            .remove_delim();
        assert_eq!("justflat", conv.convert("Just Flat"));
    }

    #[test]
    fn no_digit_boundaries() {
        let conv = Converter::new()
            .remove_boundaries(&Boundary::digits())
            .to_case(Case::Snake);
        assert_eq!("test_08bound", conv.convert("Test 08Bound"));
        assert_eq!("a8a_a8a", conv.convert("a8aA8A"));
    }
    
    #[test]
    fn add_boundary() {
        let conv = Converter::new()
            .from_case(Case::Snake)
            .to_case(Case::Kebab)
            .add_boundary(Boundary::LowerUpper);
        assert_eq!("word-word-word", conv.convert("word_wordWord"));
    }
    
    #[test]
    fn reuse_after_change() {
        let conv = Converter::new()
            .from_case(Case::Snake)
            .to_case(Case::Kebab);
        assert_eq!("word-wordword", conv.convert("word_wordWord"));

        let conv = conv.add_boundary(Boundary::LowerUpper);
        assert_eq!("word-word-word", conv.convert("word_wordWord"));
    }

    #[test]
    fn explicit_boundaries() {
        let conv = Converter::new()
            .set_boundaries(&[Boundary::DigitLower, Boundary::DigitUpper, Boundary::Acronyms])
            .to_case(Case::Snake);
        assert_eq!("section8_lesson2_http_requests", conv.convert("section8lesson2HTTPRequests"));
    }
}
