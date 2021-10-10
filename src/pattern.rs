use std::iter;

#[cfg(feature = "random")]
use rand::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum WordCase {
    Lower,
    Upper,
    Capital,
    Toggle,
}

impl WordCase {
    fn mutate(&self, word: &str) -> String {
        use WordCase::*;
        match self {
            Lower => word.to_lowercase(),
            Upper => word.to_uppercase(),
            Capital => {
                let mut chars = word.chars();
                if let Some(c) = chars.next() {
                    c.to_uppercase()
                        .chain(chars.as_str().to_lowercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            }
            Toggle => {
                let mut chars = word.chars();
                if let Some(c) = chars.next() {
                    c.to_lowercase()
                        .chain(chars.as_str().to_uppercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            }
        }
    }
}

/// A pattern is how a set of words is mutated before joining with
/// a delimeter.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Pattern {
    /// Lowercase patterns make all words lowercase.
    Lowercase,

    /// Uppercase patterns make all words uppercase.
    Uppercase,

    /// Capital patterns makes the first letter of each word uppercase
    /// and the remaining letters of each word lowercase.
    Capital,

    /// Capital patterns make the first word capitalized and the
    /// remaining lowercase.
    Sentence,

    /// Camel patterns make the first word lowercase and the remaining
    /// uppercase.
    Camel,

    /// Alternating patterns make each letter of each word alternate
    /// between lowercase and uppercase.  They alternate across words,
    /// which means the last letter of one word and the first letter of the
    /// next will not be the same letter casing.
    Alternating,

    /// Toggle patterns have the first letter of each word uppercase
    /// and the remaining letters of each word uppercase.
    Toggle,

    /// Random patterns will lowercase or uppercase each letter
    /// uniformly randomly.  This uses the `rand` crate and is only available with the "random"
    /// feature.
    #[cfg(feature = "random")]
    Random,

    /// PseudoRandom patterns are random-like patterns.  Instead of randomizing
    /// each letter individually, it mutates each pair of characters
    /// as either (Lowercase, Uppercase) or (Uppercase, Lowercase).  This generates
    /// more "random looking" words.  A consequence of this algorithm for randomization
    /// is that there will never be three consecutive letters that are all lowercase
    /// or all uppercase.  This uses the `rand` crate and is only available with the "random"
    /// feature.
    #[cfg(feature = "random")]
    PseudoRandom,
}

impl Pattern {
    /// Generates a vector of new `String`s in the right pattern given
    /// the input strings.
    /// ```
    /// use convert_case::Pattern;
    ///
    /// assert_eq!(
    ///     vec!["crack", "the", "skye"],
    ///     Pattern::Lowercase.mutate(&vec!["CRACK", "the", "Skye"]),
    /// )
    /// ```
    pub fn mutate(&self, words: &[&str]) -> Vec<String> {
        use Pattern::*;
        match self {
            Lowercase => words
                .iter()
                .map(|word| WordCase::Lower.mutate(word))
                .collect(),
            Uppercase => words
                .iter()
                .map(|word| WordCase::Upper.mutate(word))
                .collect(),
            Capital => words
                .iter()
                .map(|word| WordCase::Capital.mutate(word))
                .collect(),
            Toggle => words
                .iter()
                .map(|word| WordCase::Toggle.mutate(word))
                .collect(),
            Sentence => {
                let word_cases =
                    iter::once(WordCase::Capital).chain(iter::once(WordCase::Lower).cycle());
                words
                    .iter()
                    .zip(word_cases)
                    .map(|(word, word_case)| word_case.mutate(word))
                    .collect()
            }
            Camel => {
                let word_cases =
                    iter::once(WordCase::Lower).chain(iter::once(WordCase::Capital).cycle());
                words
                    .iter()
                    .zip(word_cases)
                    .map(|(word, word_case)| word_case.mutate(word))
                    .collect()
            }
            Alternating => alternating(words),
            #[cfg(feature = "random")]
            Random => randomize(words),
            #[cfg(feature = "random")]
            PseudoRandom => pseudo_randomize(words),
        }
    }
}

fn alternating(words: &[&str]) -> Vec<String> {
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
fn randomize(words: &[&str]) -> Vec<String> {
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
fn pseudo_randomize(words: &[&str]) -> Vec<String> {
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
            let new = pseudo_randomize(&words).join("");
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
            let transformed = pseudo_randomize(&words);
            assert_ne!(words, transformed);
            let transformed = randomize(&words);
            assert_ne!(words, transformed);
        }
    }

    #[test]
    fn mutate_empty_strings() {
        for wcase in [
            WordCase::Lower,
            WordCase::Upper,
            WordCase::Capital,
            WordCase::Toggle,
        ] {
            assert_eq!(String::new(), wcase.mutate(&String::new()))
        }
    }
}
