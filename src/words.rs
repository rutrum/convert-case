use crate::Case;

#[cfg(feature = "random")]
use rand::prelude::*;

pub(super) struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn new(name: &str) -> Self {
        let words = name
            .split(|c| "-_ ".contains(c))
            .flat_map(Self::split_camel)
            .filter(|s| !s.is_empty())
            .collect();
        Words { words }
    }

    pub fn from_casing(name: &str, case: Case) -> Self {
        use Case::*;
        let words = match case {
            Title | Upper | Lower | Toggle | Alternating => name
                .split_ascii_whitespace()
                .map(ToString::to_string)
                .collect(),
            Kebab | Cobol | UpperKebab | Train => name
                .split('-')
                .map(ToString::to_string)
                .filter(|s| !s.is_empty())
                .collect(),
            Snake | UpperSnake | ScreamingSnake => name
                .split('_')
                .map(ToString::to_string)
                .filter(|s| !s.is_empty())
                .collect(),
            Pascal | Camel | UpperCamel => Self::split_camel(name),
            Flat | UpperFlat => vec![name.to_string()],

            // Same behavior as title, upper, etc.
            #[cfg(feature = "random")]
            Random | PseudoRandom => name
                .split_ascii_whitespace()
                .map(ToString::to_string)
                .collect(),
        };
        Self { words }
    }

    fn split_camel(name: &str) -> Vec<String> {
        let left_iter = name.chars();
        let mid_iter = name.chars().skip(1);
        let right_iter = name.chars().skip(2);

        let mut split_indices = left_iter
            .zip(mid_iter)
            .zip(right_iter)
            .enumerate()
            .filter(|(_, ((f, s), t))| Self::three_char_is_boundary(*f, *s, *t))
            .map(|(i, _)| i + 1)
            .collect::<Vec<usize>>();

        // Check for boundary in the last two characters
        // Can be rewritten nicer (use fold)
        let mut backwards_seek = name.chars().rev();
        let last = backwards_seek.next();
        let second_last = backwards_seek.next();
        if let (Some(a), Some(b)) = (second_last, last) {
            if Self::two_char_is_boundary(a, b) {
                split_indices.push(name.len() - 1);
            }
        }

        Self::split_at_indices(name, split_indices)
    }

    /// Allowed boundaries are (lower upper) (digit (!digit and !punc)) ((!digit and !punc) digit).
    fn two_char_is_boundary(f: char, s: char) -> bool {
        (f.is_lowercase() && s.is_uppercase())
            || (f.is_ascii_digit() && !(s.is_ascii_digit() || s.is_ascii_punctuation()))
            || (!(f.is_ascii_digit() || f.is_ascii_punctuation()) && s.is_ascii_digit())
    }

    /// Checks if three characters are the end of an acronym, otherwise
    /// calls `two_char_is_boundary`.
    fn three_char_is_boundary(f: char, s: char, t: char) -> bool {
        (f.is_uppercase() && s.is_uppercase() && t.is_lowercase())
            || Self::two_char_is_boundary(f, s)
    }

    fn split_at_indices(name: &str, indices: Vec<usize>) -> Vec<String> {
        let mut words = Vec::new();

        let mut first = name;
        let mut second;
        for &x in indices.iter().rev() {
            let pair = first.split_at(x);
            first = pair.0;
            second = pair.1;
            words.push(second);
        }
        words.push(first);

        words.iter().rev().map(ToString::to_string).collect()
    }

    pub fn into_case(mut self, case: Case) -> String {
        let words = self.words;
        let pattern = case.pattern();
        let delim = case.delim();
        pattern.mutate(&words).join(delim)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_two_char_boundaries() {
        assert!(!Words::two_char_is_boundary('a', 'a'));
        assert!(Words::two_char_is_boundary('a', 'A'));
        assert!(Words::two_char_is_boundary('a', '5'));
        assert!(!Words::two_char_is_boundary('a', ','));
        assert!(!Words::two_char_is_boundary('A', 'A'));
        assert!(!Words::two_char_is_boundary('A', 'a'));
        assert!(Words::two_char_is_boundary('A', '5'));
        assert!(!Words::two_char_is_boundary('A', ','));
        assert!(Words::two_char_is_boundary('5', 'a'));
        assert!(Words::two_char_is_boundary('5', 'A'));
        assert!(!Words::two_char_is_boundary('5', '5'));
        assert!(!Words::two_char_is_boundary('5', ','));
        assert!(!Words::two_char_is_boundary(',', 'a'));
        assert!(!Words::two_char_is_boundary(',', 'A'));
        assert!(!Words::two_char_is_boundary(',', '5'));
        assert!(!Words::two_char_is_boundary(',', ','));
    }

    #[test]
    fn correct_three_char_boundaries() {
        assert!(Words::three_char_is_boundary('A', 'A', 'a'));
        assert!(!Words::three_char_is_boundary('A', 'a', 'a'));
        assert!(!Words::three_char_is_boundary('A', 'a', 'A'));
        assert!(!Words::three_char_is_boundary('A', 'A', '3'));
    }
}
