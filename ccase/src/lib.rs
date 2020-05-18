use convert_case::{Casing, Case};

#[cfg(test)]
use strum_macros::EnumIter;

#[cfg_attr(test, derive(EnumIter))]
#[derive(PartialEq, Eq)]
pub enum CaseKind {
    SpaceDelim,
    UnderscoreDelim,
    NoDelim,
    HyphenDelim,
    CapitalDelim,
    Random,
}

impl CaseKind {
    pub fn from_case(c: Case) -> Self {
        use Case::*;
        use CaseKind::*;
        match c {
            Upper | Lower | Title | Toggle | Alternating => SpaceDelim,
            Snake | UpperSnake | ScreamingSnake => UnderscoreDelim,
            Kebab | Cobol | Train => HyphenDelim,
            Flat | UpperFlat => NoDelim,
            Camel | UpperCamel | Pascal => CapitalDelim,
            Case::Random | PseudoRandom => CaseKind::Random,
        }
    }

    pub const fn all_kinds() -> [CaseKind; 6] {
        use CaseKind::*;
        [
            SpaceDelim,
            CapitalDelim,
            UnderscoreDelim,
            HyphenDelim,
            NoDelim,
            Random,
        ]
    }

    pub fn name(&self) -> &'static str {
        use CaseKind::*;
        match self {
            SpaceDelim => "Space Delimited",
            UnderscoreDelim => "Underscore Delimited",
            NoDelim => "No Delimiter",
            HyphenDelim => "Hyphen Delimited",
            CapitalDelim => "Capitalization Boundaries",
            Random => "Random Capitalization",
        }
    }
}

pub trait CaseClassification {
    fn name_in_case(self) -> String;
    fn is_alias(&self) -> Option<Case>;
    fn kind(&self) -> CaseKind;
    fn short_name(&self) -> Option<&'static str>;
    fn from_str(_: &str) -> Result<Case, ()>;
    fn list();
}

impl CaseClassification for Case {
    // Returns the name of the case in it's own case
    fn name_in_case(self) -> String {
        let case_str = format!("{:?}Case", self);
        case_str.to_case(self)
    }

    // Returns None if case is not an alias for another case,
    // other Some containing the other case
    fn is_alias(&self) -> Option<Case> {
        use Case::*;
        match self {
            UpperCamel => Some(Pascal),
            ScreamingSnake => Some(UpperSnake),
            _ => None
        }
    }

    // Maps a case to its cooresponding CaseKind
    fn kind(&self) -> CaseKind {
        use Case::*;
        use CaseKind::*;
        match self {
            Upper | Lower | Title | Toggle | Alternating => SpaceDelim,
            Snake | UpperSnake | ScreamingSnake => UnderscoreDelim,
            Kebab | Cobol | Train => HyphenDelim,
            Flat | UpperFlat => NoDelim,
            Camel | UpperCamel | Pascal => CapitalDelim,
            Case::Random | PseudoRandom => CaseKind::Random,
        }
    }

    // Returns the short name variant of a case name if it exists
    fn short_name(&self) -> Option<&'static str> {
        use Case::*;
        match self {
            PseudoRandom => Some("pseudo"),
            ScreamingSnake => Some("screaming"),
            Alternating => Some("alternate"),
            _ => None,
        }
    }

    // Tries to convert a string slice to a Case
    fn from_str(s: &str) -> Result<Case, ()> {
        let case_str = s.to_case(Case::Flat);
        for case in Case::all_cases() {
            if case_str == format!("{:?}", case).to_case(Case::Flat) {
                return Ok(case);
            }
            if let Some(short) = case.short_name() {
                if case_str == short {
                    return Ok(case);
                }
            }
        }
        Err(())
    }

    // Prints to the screen a list of cases
    fn list() {
        for kind in &CaseKind::all_kinds() {
            println!("{}:", kind.name());
            for case in Case::all_cases()
                .iter()
                .filter(|&x| x.kind() == *kind)
            {
                println!("  {:<16} {}", format!("{:?}", case), case.name_in_case());
                if let Some(short) = case.short_name() {
                    println!("   (or {})", short);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn all_kinds_in_iter() {
        for kind in CaseKind::iter() {
            assert!(CaseKind::all_kinds().contains(&kind));
        }
    }

    #[test]
    fn short_name_to_case() {
        assert_eq!(Case::from_str("pseudo").unwrap(), Case::PseudoRandom);
    }

}
