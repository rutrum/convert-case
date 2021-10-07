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
    fn mutate(&self, word: &String) -> String {
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

pub enum Pattern {
    /// Lowercase patterns are words that are entirely in lowercase
    Lowercase,
    Uppercase,
    Capital,
    Sentence,
    Camel,

    Alternating,
    Toggle,

    #[cfg(feature = "random")]
    Random,
    #[cfg(feature = "random")]
    PseudoRandom,
}

impl Pattern {
    fn mutate(&self, words: &Vec<String>) -> Vec<String> {
        use Pattern::*;
        match self {
            Lowercase => words.iter()
                .map(|word| WordCase::Lower.mutate(word))
                .collect(),
            Uppercase => words.iter()
                .map(|word| WordCase::Upper.mutate(word))
                .collect(),
            Capital => words.iter()
                .map(|word| WordCase::Capital.mutate(word))
                .collect(),
            Toggle => words.iter()
                .map(|word| WordCase::Toggle.mutate(word))
                .collect(),
            Sentence => {
                let word_cases = iter::once(WordCase::Capital)
                    .chain(iter::once(WordCase::Lower).cycle());
                words.iter()
                    .zip(word_cases)
                    .map(|(word, word_case)| word_case.mutate(word))
                    .collect()
            }
            Camel => {
                let word_cases = iter::once(WordCase::Lower)
                    .chain(iter::once(WordCase::Capital).cycle());
                words.iter()
                    .zip(word_cases)
                    .map(|(word, word_case)| word_case.mutate(word))
                    .collect()
            }
            Alternating => {
                let mut upper = false;
                words.iter()
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
            #[cfg(feature = "random")]
            Random => {
                randomize(words)
            }
            #[cfg(feature = "random")]
            PseudoRandom => {
                pseudo_randomize(words)
            }
        }
    }
}

/// Randomly picks whether to be upper case or lower case
#[cfg(feature = "random")]
fn randomize(words: &Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words.iter()
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
fn pseudo_randomize(words: &Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();

    // Keeps track of when to alternate
    let mut alt: Option<bool> = None;
    words.iter()
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
