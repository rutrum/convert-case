use atty::Stream;
use clap::{crate_version, App, AppSettings, Arg, ArgMatches};
use convert_case::{Case, Casing};
use std::io::{self, BufRead};
use std::fmt;

use ccase_lib::CaseClassification;

enum Error {
    Stdin,
    NoToCase,
    InvalidCase(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        let s = match self {
            Stdin => "Unable to read from stdin".to_string(),
            NoToCase => "No `to-case` provided".to_string(),
            InvalidCase(s) => format!("The `{}` case is not implemented", s)
        };
        write!(f, "{}", s)
    }
}

struct Conversion {
    to: Case,
    from: Option<Case>,
    s: Option<String>,
}

impl Conversion {
    pub fn from_matches(matches: ArgMatches) -> Result<Self, Error> {
        let to_str = matches.value_of("to-case").ok_or(Error::NoToCase)?;
        let to = Case::from_str(to_str).map_err(|_| Error::InvalidCase(to_str.to_string()))?;

        let from = match matches.value_of("from-case") {
            None => None,
            Some(from_str) => {
                Some(Case::from_str(from_str).map_err(|_| Error::InvalidCase(from_str.to_string()))?)
            }
        };

        let s = match matches.value_of("INPUT") {
            Some(s) if !s.is_empty() => Some(s.to_string()),
            _ => None
        };

        Ok(Conversion {to, from, s })
    }

    fn convert(self) -> Result<String, Error> {
        let new = match (self.from, self.s) {
            (Some(from), Some(s)) => s.from_case(from).to_case(self.to),
            (None, Some(s)) => s.to_case(self.to),
            (Some(from), None) => {
                let mut lines = vec![];
                for line in io::stdin().lock().lines() {
                    let s = line.map_err(|_| Error::Stdin)?.from_case(from).to_case(self.to);
                    lines.push(s);
                }
                lines.join("\n")
            }
            (None, None) => {
                let mut lines = vec![];
                for line in io::stdin().lock().lines() {
                    let s = line.map_err(|_| Error::Stdin)?.to_case(self.to);
                    lines.push(s);
                }
                lines.join("\n")
            }
        };
        Ok(new)
    }
}

fn main() -> Result<(), ()> {
    let app = create_app();
    let matches = app.get_matches();

    if matches.is_present("list-cases") {
        Case::list();
        return Ok(());
    }

    match Conversion::from_matches(matches) {
        Ok(c) => match c.convert() {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("{}", e);
                return Err(());
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    }
    
    Ok(())
}

fn convert_stdin(matches: ArgMatches) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {}
}

fn convert(matches: ArgMatches) {}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Convert Case")
        .version(crate_version!())
        .author("Dave Purdum <purdum41@gmail.com>")
        .about("Converts to and from various cases.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("list-cases")
                .short("l")
                .long("list")
                .help("Lists available cases"),
        )
        .arg(
            Arg::with_name("to-case")
                .short("t")
                .long("to")
                .value_name("CASE")
                .help("The case to convert into.")
                .takes_value(true)
                .required_unless("list-cases")
                .validator(is_valid_case),
        )
        .arg(
            Arg::with_name("from-case")
                .short("f")
                .long("from")
                .value_name("CASE")
                .help("The case to parse input as.")
                .validator(is_valid_case)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The string to convert.")
                //.default_value("")
                //.required_unless("list-cases")
                .requires("to-case")
                //.required(true)
                .validator(pipe_or_inline),
        )
        .arg(
            Arg::with_name("rename-file")
                .help("Rename the given file.")
                .requires("to-case")
                .short("r")
                .long("rename"),
        )
}

// Returns true if either a string is provided or data
// is being piped in from stdin
fn pipe_or_inline(s: String) -> Result<(), String> {
    let valid = !s.is_empty() || atty::isnt(Stream::Stdin);
    if valid {
        Ok(())
    } else {
        Err("require input inline or from stdin".to_string())
    }
}

fn is_valid_case(s: String) -> Result<(), String> {
    match Case::from_str(s.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("the '{}' case is not implemented", s)),
    }
}

#[cfg(test)]
mod test {

    use assert_cli::Assert;

    #[test]
    fn inline_use() {
        Assert::main_binary()
            .with_args(&["-t", "snake", "myVarName"])
            .stdout()
            .is("my_var_name")
            .unwrap();
    }

    #[test]
    fn stdin_use() {
        Assert::main_binary()
            .stdin("myVarName")
            .with_args(&["-t", "snake"])
            .stdout()
            .is("my_var_name")
            .unwrap();
    }

    #[test]
    fn help_by_default() {
        Assert::main_binary()
            .fails()
            .stderr()
            .contains("FLAGS:")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn prints_require_input() {
        Assert::main_binary()
            .with_args(&["-t", "snake"])
            .fails()
            .stderr()
            .contains("The following required arguments were not provided")
            .unwrap();
    }

    #[test]
    fn multiline_stdin() {
        Assert::main_binary()
            .stdin("one\ntwo\nthree")
            .with_args(&["-t", "title"])
            .stdout()
            .is("One\nTwo\nThree")
            .unwrap();
    }

    #[test]
    fn bad_case() {
        Assert::main_binary()
            .with_args(&["-t", "ttle", "myVarName"])
            .fails()
            .unwrap();
        Assert::main_binary()
            .with_args(&["-t", "title", "-f", "cammel", "myVarName"])
            .fails()
            .unwrap();
    }
}
