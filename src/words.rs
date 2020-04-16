use crate::Case;

pub(super) struct Words {
    words: Vec<String>,
}

impl Words {
    pub fn new(name: &str) -> Self {
        if name.matches(" ").count() > 0 {
            Self::from_casing(name, Case::Title)
        } else if name.matches("_").count() > 0 {
            Self::from_casing(name, Case::Snake)
        } else if name.matches("-").count() > 0 {
            Self::from_casing(name, Case::Kebab)
        } else {
            Self::from_casing(name, Case::Camel)
        }
    }

    pub fn from_casing(name: &str, case: Case) -> Self {
        use Case::*;
        let words = match case {
            Title | Upper | Lower | Toggle => name
                .split_ascii_whitespace()
                .map(ToString::to_string)
                .collect(),
            Kebab | Cobol | Train => name.split('-').map(ToString::to_string).collect(),
            Snake | UpperSnake | ScreamingSnake => {
                name.split('_').map(ToString::to_string).collect()
            }
            Pascal | Camel | UpperCamel => Self::split_lower_to_upper(name),
            Flat | UpperFlat => vec![name.to_string()],
        };
        Self { words }
    }

    fn split_lower_to_upper(name: &str) -> Vec<String> {
        let mut split_indices = Vec::new();

        let left_iter = name.chars();
        let right_iter = name.chars().skip(1);
        left_iter
            .zip(right_iter)
            .enumerate()
            .for_each(|(i, (a, b))| {
                if a.is_lowercase() && !b.is_lowercase() || !a.is_uppercase() && b.is_uppercase() {
                    split_indices.push(i + 1);
                }
            });

        let mut words = Vec::new();

        let mut first = name;
        let mut second;
        for &x in split_indices.iter().rev() {
            let pair = first.split_at(x);
            first = pair.0;
            second = pair.1;
            words.insert(0, second);
        }
        words.insert(0, first);

        words.iter().map(ToString::to_string).collect()
    }

    pub fn into_case(mut self, case: Case) -> String {
        use Case::*;
        match case {
            Camel => self.into_camel_case(),
            Title => {
                self.capitalize_first_letter();
                self.join(" ")
            }
            Pascal | UpperCamel => {
                self.capitalize_first_letter();
                self.join("")
            }
            Toggle => {
                self.lower_first_letter();
                self.join(" ")
            }
            Snake => {
                self.make_lowercase();
                self.join("_")
            }
            Cobol => {
                self.make_uppercase();
                self.join("-")
            }
            Kebab => {
                self.make_lowercase();
                self.join("-")
            }
            UpperSnake | ScreamingSnake => {
                self.make_uppercase();
                self.join("_")
            }
            Lower => {
                self.make_lowercase();
                self.join(" ")
            }
            Upper => {
                self.make_uppercase();
                self.join(" ")
            }
            Flat => {
                self.make_lowercase();
                self.join("")
            }
            Train => {
                self.capitalize_first_letter();
                self.join("-")
            }
            UpperFlat => {
                self.make_uppercase();
                self.join("")
            }
        }
    }

    fn into_camel_case(mut self) -> String {
        self.words = self
            .words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                if i != 0 {
                    let mut chars = word.chars();
                    if let Some(a) = chars.next() {
                        a.to_uppercase()
                            .chain(chars.as_str().to_lowercase().chars())
                            .collect()
                    } else {
                        String::new()
                    }
                } else {
                    word.to_lowercase()
                }
            })
            .collect();
        self.join("")
    }

    fn make_uppercase(&mut self) {
        self.words = self.words.iter().map(|word| word.to_uppercase()).collect();
    }

    fn make_lowercase(&mut self) {
        self.words = self.words.iter().map(|word| word.to_lowercase()).collect();
    }

    fn capitalize_first_letter(&mut self) {
        self.words = self
            .words
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                if let Some(a) = chars.next() {
                    a.to_uppercase()
                        .chain(chars.as_str().to_lowercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            })
            .collect();
    }

    fn lower_first_letter(&mut self) {
        self.words = self
            .words
            .iter()
            .map(|word| {
                let mut chars = word.chars();
                if let Some(a) = chars.next() {
                    a.to_lowercase()
                        .chain(chars.as_str().to_uppercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            })
            .collect();
    }

    // Alternative: construct [my, -, variable, -, name] then collect
    fn join(self, delim: &str) -> String {
        self.words
            .iter()
            .enumerate()
            .map(|(i, val)| {
                if i == 0 {
                    val.to_owned()
                } else {
                    delim.to_owned() + val
                }
            })
            .collect()
    }
}
