use std::iter;

#[cfg(feature = "random")]
use rand::prelude::*;

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum WordPattern {
    Graphemes(fn(Graphemes) -> String),
    Str(fn(&str) -> String),
}

impl WordPattern {
    const LOWER: Self = WordPattern::Str(|word| word.to_lowercase());
    const UPPER: Self = WordPattern::Str(|word| word.to_uppercase());
    const CAPITAL: Self = WordPattern::Graphemes(|mut graphemes| {
        if let Some(c) = graphemes.next() {
            [c.to_uppercase(), graphemes.as_str().to_lowercase()].concat()
        } else {
            String::new()
        }
    });
    const TOGGLE: Self = WordPattern::Graphemes(|mut graphemes| {
        if let Some(c) = graphemes.next() {
            [c.to_lowercase(), graphemes.as_str().to_uppercase()].concat()
        } else {
            String::new()
        }
    });

    fn mutate(&self, word: &str) -> String {
        match self {
            Self::Graphemes(f) => f(word.graphemes(true)),
            Self::Str(f) => f(word),
        }
    }
}

// if it's in a module, it would be easier

pub mod word_pattern {
    use super::*;

    pub type WordPattern = fn(&str) -> String;

    pub fn lowercase(word: &str) -> String {
        word.to_lowercase()
    }

    pub fn uppercase(word: &str) -> String {
        word.to_uppercase()
    }

    // what about graphemes?
    pub fn capital(word: &str) -> String {
        let mut graphemes = word.graphemes(true);

        if let Some(c) = graphemes.next() {
            [c.to_uppercase(), graphemes.as_str().to_lowercase()].concat()
        } else {
            String::new()
        }
    }

    pub fn toggle(word: &str) -> String {
        let mut graphemes = word.graphemes(true);

        if let Some(c) = graphemes.next() {
            [c.to_lowercase(), graphemes.as_str().to_uppercase()].concat()
        } else {
            String::new()
        }
    }

    // this feels better than Pattern as a enum, definitely better than struct
    // oh no...is the same true for boundary?
}

// used like this

// Can't just make struct callable.
// pub struct PatternFn;
// impl Fn<Args> for PatternFn {
// }

// But I could just make a pattern type

pub type Pattern = fn(&[&str]) -> Vec<String>;

pub fn noop(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

pub fn lowercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::lowercase(word))
        .collect()
}

pub fn uppercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::uppercase(word))
        .collect()
}

pub fn capital(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::capital(word))
        .collect()
}

pub fn camel(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| {
            if i == 0 {
                word_pattern::lowercase(&word)
            } else {
                word_pattern::capital(&word)
            }
        })
        .collect()
}

pub fn sentence(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| {
            if i == 0 {
                word_pattern::capital(&word)
            } else {
                word_pattern::lowercase(&word)
            }
        })
        .collect()
}

pub fn toggle(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::toggle(word))
        .collect()
}

pub fn alternating(words: &[&str]) -> Vec<String> {
    let mut upper = false;
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    if letter.is_uppercase() || letter.is_lowercase() {
                        if upper {
                            upper = false;
                            letter.to_uppercase().to_string()
                        } else {
                            upper = true;
                            letter.to_lowercase().to_string()
                        }
                    } else {
                        letter.to_string()
                    }
                })
                .collect()
        })
        .collect()
}

/// Randomly picks whether to be upper case or lower case
#[cfg(feature = "random")]
pub fn random(words: &[&str]) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    if rng.gen::<f32>() > 0.5 {
                        letter.to_uppercase().to_string()
                    } else {
                        letter.to_lowercase().to_string()
                    }
                })
                .collect()
        })
        .collect()
}

/// Randomly selects patterns: [upper, lower] or [lower, upper]
/// for a more random feeling pattern.
#[cfg(feature = "random")]
pub fn pseudo_random(words: &[&str]) -> Vec<String> {
    let mut rng = rand::thread_rng();

    // Keeps track of when to alternate
    let mut alt: Option<bool> = None;
    words
        .iter()
        .map(|word| {
            word.chars()
                .map(|letter| {
                    match alt {
                        // No existing pattern, start one
                        None => {
                            if rng.gen::<f32>() > 0.5 {
                                alt = Some(false); // Make the next char lower
                                letter.to_uppercase().to_string()
                            } else {
                                alt = Some(true); // Make the next char upper
                                letter.to_lowercase().to_string()
                            }
                        }
                        // Existing pattern, do what it says
                        Some(upper) => {
                            alt = None;
                            if upper {
                                letter.to_uppercase().to_string()
                            } else {
                                letter.to_lowercase().to_string()
                            }
                        }
                    }
                })
                .collect()
        })
        .collect()
}

/*

// this works, but makes less sense for word_pattern
// I think I did the whole graphemes/str thing just so I could
// allow a user to traverse over graphemes without having
// to import the unicode_segmentation module

#[derive(Debug, Eq, PartialEq, Clone, Hash, Copy)]
/// Patterns mutate the case of a series of words.
pub struct Pattern(fn(&[&str]) -> Vec<String>);

impl Pattern {
    /// The no-op pattern performs no mutations.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["Case", "CONVERSION", "library"],
    ///     Pattern::NOOP.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const NOOP: Self = Pattern(|words| words.iter().map(ToString::to_string).collect());

    /// Lowercase patterns make all words lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["case", "conversion", "library"],
    ///     Pattern::LOWERCASE.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const LOWERCASE: Self = Pattern(|words| {
        words
            .iter()
            .map(|word| WordPattern::LOWER.mutate(&word))
            .collect()
    });

    /// Uppercase patterns make all words uppercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["CASE", "CONVERSION", "LIBRARY"],
    ///     Pattern::UPPERCASE.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const UPPERCASE: Self = Pattern(|words| {
        words
            .iter()
            .map(|word| WordPattern::UPPER.mutate(&word))
            .collect()
    });

    /// Capital patterns makes the first letter of each word uppercase
    /// and the remaining letters of each word lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["Case", "Conversion", "Library"],
    ///     Pattern::CAPITAL.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const CAPITAL: Self = Pattern(|words| {
        words
            .iter()
            .map(|word| WordPattern::CAPITAL.mutate(&word))
            .collect()
    });

    /// Camel patterns make the first word lowercase and the remaining
    /// capitalized.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["case", "Conversion", "Library"],
    ///     Pattern::CAMEL.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const CAMEL: Self = Pattern(|words| {
        let word_patterns =
            iter::once(WordPattern::LOWER).chain(iter::once(WordPattern::CAPITAL).cycle());
        words
            .iter()
            .zip(word_patterns)
            .map(|(word, pattern)| pattern.mutate(word))
            .collect()
    });

    /// Capital patterns make the first word capitalized and the
    /// remaining lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["Case", "conversion", "library"],
    ///     Pattern::SENTENCE.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const SENTENCE: Self = Pattern(|words| {
        let word_patterns =
            iter::once(WordPattern::CAPITAL).chain(iter::once(WordPattern::LOWER).cycle());
        words
            .iter()
            .zip(word_patterns)
            .map(|(word, pattern)| pattern.mutate(word))
            .collect()
    });

    /// Alternating patterns make each letter of each word alternate
    /// between lowercase and uppercase.  They alternate across words,
    /// which means the last letter of one word and the first letter of the
    /// next will not be the same letter casing.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["cAsE", "cOnVeRsIoN", "lIbRaRy"],
    ///     Pattern::ALTERNATING.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// assert_eq!(
    ///     vec!["aNoThEr", "ExAmPlE"],
    ///     Pattern::ALTERNATING.mutate(&["Another", "Example"]),
    /// );
    /// ```
    pub const ALTERNATING: Self = Pattern(|words| alternating(words));

    /// Toggle patterns have the first letter of each word uppercase
    /// and the remaining letters of each word uppercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     vec!["cASE", "cONVERSION", "lIBRARY"],
    ///     Pattern::TOGGLE.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    pub const TOGGLE: Self = Pattern(|words| {
        words
            .iter()
            .map(|word| WordPattern::TOGGLE.mutate(&word))
            .collect()
    });

    /// Random patterns will lowercase or uppercase each letter
    /// uniformly randomly.  This uses the `rand` crate and is only available with the "random"
    /// feature.  This example will not pass the assertion due to randomness, but it used as an
    /// example of what output is possible.
    /// ```should_panic
    /// # use convert_case::Pattern;
    /// # #[cfg(any(doc, feature = "random"))]
    /// assert_eq!(
    ///     vec!["Case", "coNVeRSiOn", "lIBraRY"],
    ///     Pattern::RANDOM.mutate(&["Case", "CONVERSION", "library"])
    /// );
    /// ```
    #[cfg(feature = "random")]
    pub const RANDOM: Self = Pattern(|words| randomize(words));

    /// PseudoRandom patterns are random-like patterns.  Instead of randomizing
    /// each letter individually, it mutates each pair of characters
    /// as either (Lowercase, Uppercase) or (Uppercase, Lowercase).  This generates
    /// more "random looking" words.  A consequence of this algorithm for randomization
    /// is that there will never be three consecutive letters that are all lowercase
    /// or all uppercase.  This uses the `rand` crate and is only available with the "random"
    /// feature.  This example will not pass the assertion due to randomness, but it used as an
    /// example of what output is possible.
    /// ```should_panic
    /// # use convert_case::Pattern;
    /// # #[cfg(any(doc, feature = "random"))]
    /// assert_eq!(
    ///     vec!["cAsE", "cONveRSioN", "lIBrAry"],
    ///     Pattern::PSEUDO_RANDOM.mutate(&["Case", "CONVERSION", "library"]),
    /// );
    /// ```
    #[cfg(feature = "random")]
    pub const PSEUDO_RANDOM: Self = Pattern(|words| pseudo_randomize(words));

    pub fn mutate(self, words: &[&str]) -> Vec<String> {
        self.0(words)
    }

    // constructor
    // from iterator of word patterns
    // like camel and sentence
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "random")]
    #[test]
    fn pseudo_no_triples() {
        let words = vec!["abcdefg", "hijklmnop", "qrstuv", "wxyz"];
        for _ in 0..5 {
            let new = pseudo_random(&words).join("");
            let mut iter = new
                .chars()
                .zip(new.chars().skip(1))
                .zip(new.chars().skip(2));
            assert!(!iter
                .clone()
                .any(|((a, b), c)| a.is_lowercase() && b.is_lowercase() && c.is_lowercase()));
            assert!(
                !iter.any(|((a, b), c)| a.is_uppercase() && b.is_uppercase() && c.is_uppercase())
            );
        }
    }

    #[cfg(feature = "random")]
    #[test]
    fn randoms_are_random() {
        let words = vec!["abcdefg", "hijklmnop", "qrstuv", "wxyz"];

        for _ in 0..5 {
            let transformed = pseudo_random(&words);
            assert_ne!(words, transformed);
            let transformed = random(&words);
            assert_ne!(words, transformed);
        }
    }

    #[test]
    fn mutate_empty_strings() {
        for word_pattern in [
            WordPattern::LOWER,
            WordPattern::UPPER,
            WordPattern::CAPITAL,
            WordPattern::TOGGLE,
        ] {
            assert_eq!(String::new(), word_pattern.mutate(&String::new()))
        }
    }
}
