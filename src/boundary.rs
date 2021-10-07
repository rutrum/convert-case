pub enum Boundary {
    Hyphen,
    Underscore,
    Space,

    UpperLower,
    LowerUpper,
    DigitUpper,
    UpperDigit,
    DigitLower,
    LowerDigit,
    //TwoChar(Box<dyn Fn(char, char) -> bool>),

    //UpperUpperLower, // Acronyms
    Acronyms,
    //ThreeChar(Box<dyn Fn(char, char, char) -> bool>), // more complex, should include index
}

impl Boundary {
    fn detect_one(&self, c: char) -> bool {
        use Boundary::*;
        match self {
            Hyphen => c == '-',
            Underscore => c == '_',
            Space => c == ' ',
            _ => false,
        }
    }

    fn detect_two(&self, c: char, d: char) -> bool {
        use Boundary::*;
        match self {
            UpperLower => c.is_uppercase() && d.is_lowercase(),
            LowerUpper => c.is_lowercase() && d.is_uppercase(),
            DigitUpper => c.is_ascii_digit() && d.is_uppercase(),
            UpperDigit => c.is_uppercase() && d.is_ascii_digit(),
            DigitLower => c.is_ascii_digit() && d.is_lowercase(),
            LowerDigit => c.is_lowercase() && d.is_ascii_digit(),
            _ => false,
        }
    }

    fn detect_three(&self, c: char, d: char, e: char) -> bool {
        use Boundary::*;
        match self {
            Acronyms => c.is_uppercase() && d.is_uppercase() && e.is_uppercase(),
            _ => false,
        }
    }
}

// function on String?
fn split(s: String, boundaries: Vec<Boundary>) -> Vec<String> {
    let left_iter = s.chars();
    let mid_iter = s.chars().skip(1);
    let right_iter = s.chars().skip(2);

    let three_iter = left_iter
        .zip(mid_iter)
        .zip(right_iter);

    let splits: Vec<usize> = three_iter.enumerate()
        .filter(|(_, ((c,d),e))| boundaries.iter().all(|b| b.detect_three(*c, *d, *e)))
        .map(|(i, _)| i + 2)
        .collect();

    let mut words = Vec::new();

    let mut first = s.as_str();
    let mut second;
    for &x in splits.iter().rev() {
        let pair = first.split_at(x);
        first = pair.0;
        second = pair.1;
        words.push(second);
    }
    words.push(first);

    words.iter().rev().map(ToString::to_string).collect()
}

// A boundary is either a replacement or not, maybe its Option<(usize, usize)>, where each
// index is what part to extract to make the word boundary.  If there is no replacement then
// its both are the same

