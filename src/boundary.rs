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
    
}

// A boundary is either a replacement or not, maybe its Option<(usize, usize)>, where each
// index is what part to extract to make the word boundary.  If there is no replacement then
// its both are the same

