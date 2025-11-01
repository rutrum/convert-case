//! Functions for transforming a list of words.
//!
//! A pattern is a function that maps a list of words into another list
//! after changing the casing of each letter.  How a patterns mutates
//! each letter can be dependent on the word the letters are present in.
//!
//! ## Custom Pattern
//!
//! Any function that matches the `Pattern` type alias is sufficient for
//! a pattern for custom casing behavior.  One example might be a pattern
//! that detects a fixed list of two-letter acronyms, and capitalizes them
//! appropriately on output.
//!
//! To work over graphemes, you can use the provided methods in the `word_pattern`
//! module that will handle that for you.
//! ```
//! use convert_case::{Converter, Pattern, pattern::word_pattern};
//!
//! fn pascal_upper_acronyms(words: &[&str]) -> Vec<String> {
//!     words.iter()
//!         .map(|word| word_pattern::capital(word))
//!         .map(|word| match word.as_ref() {
//!             "Io" | "Xml" => word.to_uppercase(),
//!             _ => word,
//!         })
//!         .collect()
//! }
//!
//! let acronym_converter = Converter::new()
//!     .set_pattern(Pattern::Custom(pascal_upper_acronyms));
//!
//! assert_eq!(acronym_converter.convert("io_stream"), "IOStream");
//! assert_eq!(acronym_converter.convert("xml request"), "XMLRequest");
//! ```
//!
//! Another example might be a one that explicitly adds leading
//! and trailing double underscores.  We do this by modifying the words directly,
//! which will get passed as-is to the join function.
//! ```
//! use convert_case::{Converter, Pattern};
//!
//! fn snake_dunder(mut words: &[&str]) -> Vec<String> {
//!     words
//!         .into_iter()
//!         .map(|word| word.to_lowercase())
//!         .enumerate()
//!         .map(|(i, word)| {
//!             if words.len() == 1 {
//!                 format!("__{}__", word)
//!             } else if i == 0 {
//!                 format!("__{}", word)
//!             } else if i == words.len() - 1 {
//!                 format!("{}__", word)
//!             } else {
//!                 word
//!             }
//!         })
//!         .collect()
//! }
//!
//! let dunder_converter = Converter::new()
//!     .set_pattern(Pattern::Custom(snake_dunder))
//!     .set_delim("_");
//!
//! assert_eq!(dunder_converter.convert("getAttr"), "__get_attr__");
//! assert_eq!(dunder_converter.convert("ITER"), "__iter__");
//! ```

#[cfg(feature = "random")]
use rand::prelude::*;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use unicode_segmentation::UnicodeSegmentation;

/// Methods for applying grapheme-level uppercasing and lowercasing.
///
/// You do not want to act on chars, since you could split a unicode character
/// in half and cause a panic.
pub mod word_pattern {
    use super::*;

    pub(crate) fn lowercase(word: &str) -> String {
        word.to_lowercase()
    }

    pub(crate) fn uppercase(word: &str) -> String {
        word.to_uppercase()
    }

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
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Custom(fn(&[&str]) -> Vec<String>),
    Noop,
    Lowercase,
    Uppercase,
    Capital,
    Camel,
    Sentence,
    Toggle,
    Alternating,
    #[cfg(feature = "random")]
    Random,
    #[cfg(feature = "random")]
    PseudoRandom,
}

impl Pattern {
    pub fn mutate(&self, words: &[&str]) -> Vec<String> {
        use Pattern::*;
        match self {
            Custom(_mutate) => (_mutate)(words),
            Noop => words.iter().map(|word| word.to_string()).collect(),
            Lowercase => words
                .iter()
                .map(|word| word_pattern::lowercase(word))
                .collect(),
            Uppercase => words
                .iter()
                .map(|word| word_pattern::uppercase(word))
                .collect(),
            Capital => words
                .iter()
                .map(|word| word_pattern::capital(word))
                .collect(),
            Camel => words
                .iter()
                .enumerate()
                .map(|(i, &word)| {
                    if i == 0 {
                        word_pattern::lowercase(&word)
                    } else {
                        word_pattern::capital(&word)
                    }
                })
                .collect(),
            Sentence => words
                .iter()
                .enumerate()
                .map(|(i, &word)| {
                    if i == 0 {
                        word_pattern::capital(&word)
                    } else {
                        word_pattern::lowercase(&word)
                    }
                })
                .collect(),
            Toggle => words
                .iter()
                .map(|word| word_pattern::toggle(word))
                .collect(),
            Alternating => {
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
            // #[doc(cfg(feature = "random"))]
            #[cfg(feature = "random")]
            Random => {
                // TODO: this is broken, hasn't been updated for graphemes
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
            #[cfg(feature = "random")]
            PsuedoRandom => {
                // This is a dumb feature.  Can this be seen as a custom variant?
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
        }
    }
}

/// A pattern is a function that maps a list of word references
/// to a vector of strings.  For more information
/// about patterns, see the [`pattern`](index.html) module documentation.
// pub type Pattern = fn(&[&str]) -> Vec<String>;

// TODO: do I keep all these functions?

/// The no-op pattern performs no mutations.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::noop(&["Case", "CONVERSION", "library"]),
///     vec!["Case", "CONVERSION", "library"],
/// );
/// ```
pub fn noop(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// Makes all words lowercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::lowercase(&["Case", "CONVERSION", "library"]),
///     vec!["case", "conversion", "library"],
/// );
/// ```
pub fn lowercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::lowercase(word))
        .collect()
}

/// Makes all words uppercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::uppercase(&["Case", "CONVERSION", "library"]),
///     vec!["CASE", "CONVERSION", "LIBRARY"],
/// );
/// ```
pub fn uppercase(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::uppercase(word))
        .collect()
}

/// Makes the first letter of each word uppercase
/// and the remaining letters of each word lowercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::capital(&["Case", "CONVERSION", "library"]),
///     vec!["Case", "Conversion", "Library"],
/// );
/// ```
pub fn capital(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::capital(word))
        .collect()
}

/// Makes the first word lowercase and the
/// remaining capitalized.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::camel(&["Case", "CONVERSION", "library"]),
///     vec!["case", "Conversion", "Library"],
/// );
/// ```
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

/// Makes the first word capitalized and the
/// remaining lowercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::sentence(&["Case", "CONVERSION", "library"]),
///     vec!["Case", "conversion", "library"],
/// );
/// ```
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

/// Makes the first letter of each word lowercase
/// and the remaining letters of each word uppercase.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::toggle(&["Case", "CONVERSION", "library"]),
///     vec!["cASE", "cONVERSION", "lIBRARY"],
/// );
/// ```
pub fn toggle(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| word_pattern::toggle(word))
        .collect()
}

/// Makes each letter of each word alternate
/// between lowercase and uppercase.  
///
/// It alternates across words,
/// which means the last letter of one word and the first letter of the
/// next will not be the same letter casing.
/// ```
/// # use convert_case::pattern;
/// assert_eq!(
///     pattern::alternating(&["Case", "CONVERSION", "library"]),
///     vec!["cAsE", "cOnVeRsIoN", "lIbRaRy"],
/// );
/// assert_eq!(
///     pattern::alternating(&["Another", "Example"]),
///     vec!["aNoThEr", "ExAmPlE"],
/// );
/// ```
pub fn alternating(words: &[&str]) -> Vec<String> {
    // TODO: this is broken, hasn't been updated for graphemes
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

/// Lowercases or uppercases each letter
/// uniformly randomly.  
///
/// This uses the `rand` crate and is only available with the "random" feature.
/// ```
/// # use convert_case::pattern;
/// # #[cfg(any(doc, feature = "random"))]
/// pattern::random(&["Case", "CONVERSION", "library"]);
/// // "casE", "coNVeRSiOn", "lIBraRY"
/// ```
// #[doc(cfg(feature = "random"))]
#[cfg(feature = "random")]
pub fn random(words: &[&str]) -> Vec<String> {
    // TODO: this is broken, hasn't been updated for graphemes
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

/// Case each letter in random-like patterns.  
///
/// Instead of randomizing
/// each letter individually, it mutates each pair of characters
/// as either (Lowercase, Uppercase) or (Uppercase, Lowercase).  This generates
/// more "random looking" words.  A consequence of this algorithm for randomization
/// is that there will never be three consecutive letters that are all lowercase
/// or all uppercase.  This uses the `rand` crate and is only available with the "random"
/// feature.
/// ```
/// # use convert_case::pattern;
/// # #[cfg(any(doc, feature = "random"))]
/// pattern::pseudo_random(&["Case", "CONVERSION", "library"]);
/// // "cAsE", "cONveRSioN", "lIBrAry"
/// ```
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
            word_pattern::lowercase,
            word_pattern::uppercase,
            word_pattern::capital,
            word_pattern::toggle,
        ] {
            assert_eq!(String::new(), word_pattern(&String::new()))
        }
    }
}
