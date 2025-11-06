#[cfg(feature = "random")]
use rand::prelude::*;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

fn lowercase_word(word: &str) -> String {
    word.to_lowercase()
}

fn uppercase_word(word: &str) -> String {
    word.to_uppercase()
}

/// Applies toggle pattern to a single word using graphemes.
fn toggle_word(word: &str) -> String {
    let mut chars = word.chars();

    if let Some(c) = chars.next() {
        [c.to_lowercase().collect(), chars.as_str().to_uppercase()].concat()
    } else {
        String::new()
    }
}

/// Applies capital pattern to a single word using graphemes.
fn capital_word(word: &str) -> String {
    let mut chars = word.chars();

    if let Some(c) = chars.next() {
        [c.to_uppercase().collect(), chars.as_str().to_lowercase()].concat()
    } else {
        String::new()
    }
}

/// Transformations on a list of words.
///
/// A pattern is a function that maps a list of words into another list
/// after changing the casing of each letter.  How a patterns mutates
/// each letter can be dependent on the word the letters are present in.
///
/// ## Custom Pattern
///
/// A pattern is a function that maps from a borrowed list of words `&[&str]` to
/// an owned list of owned words `Vec<String>` by applying a transformation.
/// One example of custom behavior might be a pattern
/// that detects a fixed list of two-letter acronyms, and capitalizes them
/// appropriately on output.
/// ```
/// use convert_case::{Converter, Pattern};
///
/// fn pascal_upper_acronyms(words: &[&str]) -> Vec<String> {
///     Pattern::Capital.mutate(words).into_iter()
///         .map(|word| match word.as_ref() {
///             "Io" | "Xml" => word.to_uppercase(),
///             _ => word,
///         })
///         .collect()
/// }
///
/// let acronym_converter = Converter::new()
///     .set_pattern(Pattern::Custom(pascal_upper_acronyms));
///
/// assert_eq!(acronym_converter.convert("io_stream"), "IOStream");
/// assert_eq!(acronym_converter.convert("xml request"), "XMLRequest");
/// ```
///
/// Another example might be a one that explicitly adds leading
/// and trailing double underscores.  We do this by modifying the words directly,
/// which will get passed as-is to the join function.
/// ```
/// use convert_case::{Converter, Pattern};
///
/// fn snake_dunder(mut words: &[&str]) -> Vec<String> {
///     words
///         .into_iter()
///         .map(|word| word.to_lowercase())
///         .enumerate()
///         .map(|(i, word)| {
///             if words.len() == 1 {
///                 format!("__{}__", word)
///             } else if i == 0 {
///                 format!("__{}", word)
///             } else if i == words.len() - 1 {
///                 format!("{}__", word)
///             } else {
///                 word
///             }
///         })
///         .collect()
/// }
///
/// let dunder_converter = Converter::new()
///     .set_pattern(Pattern::Custom(snake_dunder))
///     .set_delim("_");
///
/// assert_eq!(dunder_converter.convert("getAttr"), "__get_attr__");
/// assert_eq!(dunder_converter.convert("ITER"), "__iter__");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    /// The no-op pattern performs no mutations.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Noop.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["Case", "CONVERSION", "library"],
    /// );
    /// ```
    Noop,

    /// Makes all words lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Lowercase.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["case", "conversion", "library"],
    /// );
    /// ```
    Lowercase,

    /// Makes all words uppercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Uppercase.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["CASE", "CONVERSION", "LIBRARY"],
    /// );
    /// ```
    Uppercase,

    /// Makes the first letter of each word uppercase
    /// and the remaining letters of each word lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Capital.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["Case", "Conversion", "Library"],
    /// );
    /// ```
    Capital,

    /// Makes the first non-empty word lowercase and the
    /// remaining capitalized.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Camel.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["case", "Conversion", "Library"],
    /// );
    /// ```
    Camel,

    /// Makes the first non-empty word capitalized and the
    /// remaining lowercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Sentence.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["Case", "conversion", "library"],
    /// );
    /// ```
    Sentence,

    /// Makes the first letter of each word lowercase
    /// and the remaining letters of each word uppercase.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Toggle.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["cASE", "cONVERSION", "lIBRARY"],
    /// );
    /// ```
    Toggle,

    /// Makes each letter of each word alternate
    /// between lowercase and uppercase.
    ///
    /// It alternates across words,
    /// which means the last letter of one word and the first letter of the
    /// next will not be the same letter casing.
    /// ```
    /// # use convert_case::Pattern;
    /// assert_eq!(
    ///     Pattern::Alternating.mutate(&["Case", "CONVERSION", "library"]),
    ///     vec!["cAsE", "cOnVeRsIoN", "lIbRaRy"],
    /// );
    /// assert_eq!(
    ///     Pattern::Alternating.mutate(&["Another", "Example"]),
    ///     vec!["aNoThEr", "ExAmPlE"],
    /// );
    /// ```
    Alternating,

    // #[doc(cfg(feature = "random"))]
    /// Lowercases or uppercases each letter
    /// uniformly randomly.
    ///
    /// This uses the `rand` crate and is only available with the "random" feature.
    /// ```
    /// # use convert_case::Pattern;
    /// # #[cfg(any(doc, feature = "random"))]
    /// Pattern::Random.mutate(&["Case", "CONVERSION", "library"]);
    /// // "casE", "coNVeRSiOn", "lIBraRY"
    /// ```
    #[cfg(feature = "random")]
    Random,

    /// Case each letter in random-like patterns.
    ///
    /// Instead of randomizing
    /// each letter individually, it mutates each pair of characters
    /// as either (Lowercase, Uppercase) or (Uppercase, Lowercase).  This generates
    /// more "random looking" words.  A consequence of this algorithm for randomization
    /// is that there will never be three consecutive letters that are all lowercase
    /// or all uppercase.  This uses the `rand` crate and is only available with the "random"
    /// feature.
    ///
    /// This uses the `rand` crate and is only available with the "random" feature.
    /// ```
    /// # use convert_case::Pattern;
    /// # #[cfg(any(doc, feature = "random"))]
    /// Pattern::PseudoRandom.mutate(&["Case", "CONVERSION", "library"]);
    /// // "cAsE", "cONveRSioN", "lIBrAry"
    /// ```
    #[cfg(feature = "random")]
    PseudoRandom,

    /// Define custom behavior to transform a set of words.
    ///
    /// See the [`Pattern`] documentation for examples.
    Custom(fn(&[&str]) -> Vec<String>),
}

impl Pattern {
    /// Applies the pattern transformation to a list of words.
    pub fn mutate(&self, words: &[&str]) -> Vec<String> {
        use Pattern::*;
        match self {
            Custom(transformation) => (transformation)(words),
            Noop => words.iter().map(|word| word.to_string()).collect(),
            Lowercase => words.iter().map(|word| lowercase_word(word)).collect(),
            Uppercase => words.iter().map(|word| uppercase_word(word)).collect(),
            Capital => words.iter().map(|word| capital_word(word)).collect(),
            Camel => words
                .iter()
                .enumerate()
                .map(|(i, &word)| {
                    if i == 0 {
                        lowercase_word(word)
                    } else {
                        capital_word(word)
                    }
                })
                .collect(),
            Sentence => words
                .iter()
                .enumerate()
                .map(|(i, &word)| {
                    if i == 0 {
                        capital_word(word)
                    } else {
                        lowercase_word(word)
                    }
                })
                .collect(),
            Toggle => words.iter().map(|word| toggle_word(word)).collect(),
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

#[cfg(test)]
mod test {
    use crate::Case;
    use crate::Converter;

    use super::*;

    #[cfg(feature = "random")]
    #[test]
    fn pseudo_no_triples() {
        let words = vec!["abcdefg", "hijklmnop", "qrstuv", "wxyz"];
        for _ in 0..5 {
            let new = Pattern::PseudoRandom.mutate(&words).join("");
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
            let transformed = Pattern::PseudoRandom.mutate(&words);
            assert_ne!(words, transformed);
            let transformed = Pattern::Random.mutate(&words);
            assert_ne!(words, transformed);
        }
    }

    #[test]
    fn mutate_empty_strings() {
        for word_pattern in [lowercase_word, uppercase_word, capital_word, toggle_word] {
            assert_eq!(String::new(), word_pattern(&String::new()))
        }
    }

    #[test]
    fn filtering_with_custom() {
        // TODO: find a way to make this cleaner, then add in docs
        let filter_camel_pattern = Pattern::Custom(|words| {
            Pattern::Camel.mutate(
                &words
                    .into_iter()
                    .filter(|word| word.len() > 0)
                    .map(|word| *word)
                    .collect::<Vec<&str>>(),
            )
        });

        let conv = Converter::new()
            .from_case(Case::Kebab)
            .set_pattern(filter_camel_pattern);

        assert_eq!(conv.convert("--leading-delims"), "leadingDelims");
    }
}
