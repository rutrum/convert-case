use std::fmt;
use atty::Stream;

pub mod conversion;
pub mod list;
pub mod file;
pub use list::{CaseClassification, CaseKind};

use clap::Arg;
use convert_case::Case;

pub fn is_valid_case(s: String) -> Result<(), String> {
    match Case::from_str(s.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("the '{}' case is not implemented", s)),
    }
}


pub fn case_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("to-case")
            .short("t")
            .long("to")
            .value_name("CASE")
            .help("The case to convert into.")
            .takes_value(true)
            .validator(is_valid_case),
        Arg::with_name("from-case")
            .short("f")
            .long("from")
            .value_name("CASE")
            .help("The case to parse input as.")
            .validator(is_valid_case)
            .takes_value(true),
    ]
}


// Returns true if either a string is provided or data
// is being piped in from stdin
pub fn pipe_or_inline(s: String) -> Result<(), String> {
    let valid = !s.is_empty() || atty::isnt(Stream::Stdin);
    if valid {
        Ok(())
    } else {
        Err("require input inline or from stdin".to_string())
    }
}


#[derive(Debug)]
pub enum Error {
    Stdin,
    NoToCase,
    InvalidCase(String),
    //FileError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        let s = match self {
            Stdin => "Unable to read from stdin".to_string(),
            NoToCase => "No `to-case` provided".to_string(),
            InvalidCase(s) => format!("The `{}` case is not implemented", s),
            //FileError(s) => format!("File `{}` does not exist", s),
        };
        write!(f, "{}", s)
    }
}

