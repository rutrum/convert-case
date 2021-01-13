use atty::Stream;
use clap::{crate_version, SubCommand, App, AppSettings, Arg, ArgMatches};
use convert_case::{Case, Casing};
use std::fmt;
use std::io::{self, BufRead};

use ccase_lib::CaseClassification;

#[derive(Debug)]
enum Error {
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

struct Conversion {
    to: Case,
    from: Option<Case>,
    strings: Vec<String>,
    converted: Vec<String>,
}

use std::path::Path;

impl Conversion {
    pub fn strings(matches: &ArgMatches) -> Result<Self, Error> {
        let to = Self::to_from_matches(&matches)?;
        let from = Self::from_from_matches(&matches)?;
        let strings = Self::input_from_matches_or_stdin(&matches, "INPUT")?;
        
        let converted = strings.iter().map(|s| {
            match from {
                Some(from) => s.from_case(from).to_case(to),
                None => s.to_case(to),
            }
        }).collect();
        
        Ok(Conversion {
            to, from, strings, converted
        })
    }

    pub fn paths(matches: &ArgMatches) -> Result<Self, Error> {
        let to = Self::to_from_matches(&matches)?;
        let from = Self::from_from_matches(&matches)?;
        let strings = Self::input_from_matches_or_stdin(&matches, "PATH")?;
        
        let converted = strings.iter().map(|s| {
            let p = Path::new(s);
            let filename = p
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let new_filename = match from {
                Some(from) => filename.from_case(from).to_case(to),
                None => filename.to_case(to),
            };
            match (p.parent(), p.extension()) {
                (Some(p), Some(e)) => {
                    let mut p = p.join(new_filename).to_path_buf();
                    p.set_extension(e);
                    format!("{}", p.into_os_string().into_string().unwrap())
                }
                (Some(p), None) => format!(
                    "{}",
                    p.join(new_filename).into_os_string().into_string().unwrap()
                ),
                (None, Some(e)) => {
                    let mut p = Path::new(&new_filename).to_path_buf();
                    p.set_extension(e);
                    format!("{}", p.into_os_string().into_string().unwrap())
                }
                (None, None) => new_filename,
            }
        }).collect();
        
        Ok(Conversion {
            to, from, strings, converted
        })
    }

    fn to_from_matches(matches: &ArgMatches) -> Result<Case, Error> {
        let to_str = matches.value_of("to-case").ok_or(Error::NoToCase)?;
        let to = Case::from_str(to_str).map_err(|_| Error::InvalidCase(to_str.to_string()))?;
        Ok(to)
    }

    fn from_from_matches(matches: &ArgMatches) -> Result<Option<Case>, Error> {
        let from = match matches.value_of("from-case") {
            None => None,
            Some(from_str) => Some(
                Case::from_str(from_str).map_err(|_| Error::InvalidCase(from_str.to_string()))?,
            ),
        };
        Ok(from)
    }

    fn input_from_matches_or_stdin(matches: &ArgMatches, input: &'static str) -> Result<Vec<String>, Error> {
        let strings = match matches.value_of(input) {
            Some(s) if !s.is_empty() => vec![s.to_string()],
            _ => {
                let mut lines = vec![];
                for line in io::stdin().lock().lines() {
                    lines.push(line.map_err(|_| Error::Stdin)?);
                }
                lines
            }
        };
        Ok(strings)
    }
}

fn main() -> Result<(), Error> {
    let app = create_app();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", _) => { 
            Case::list(); 
            return Ok(());
        }
        ("file", Some(sub_m)) => {
            let conversion = Conversion::paths(&sub_m)?;
            for (o, n) in conversion.strings.iter().zip(conversion.converted.iter()) {
                match std::fs::rename(o, n) {
                    Ok(_) => println!("{} => {}", o, n),
                    Err(e) => {
                        println!("{}", e);
                        return Ok(())
                    }
                }
            }
        }
        _ => {
            // No special command, just convert input
            let conversion = Conversion::strings(&matches)?;
            for s in conversion.converted {
                println!("{}", s);
            }
        }
    }

    Ok(())
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Convert Case")
        .version(crate_version!())
        .author("Dave Purdum <purdum41@gmail.com>")
        .about("Converts to and from various cases.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("file")
                .about("Rename files in new cases")
                .setting(AppSettings::ArgRequiredElseHelp)
                .version(crate_version!())
                .author("Dave Purdum <purdum41@gmail.com>")
                .about("Renames files into various cases.")
                .arg(
                    Arg::with_name("PATH")
                        .help("The path to the file to rename.")
                        //.default_value("")
                        .requires("to-case")
                        //.required(true)
                        .validator(pipe_or_inline),
                )
                .args(&case_args())

        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List available cases")
        )
        .arg(
            Arg::with_name("INPUT")
                .help("The string to convert.")
                //.default_value("")
                .requires("to-case")
                //.required(true)
                .validator(pipe_or_inline),
        )
        .args(&case_args())
}

fn case_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("to-case")
            .short("t")
            .long("to")
            .value_name("CASE")
            .help("The case to convert into.")
            .takes_value(true)
            //.required_unless("list-cases")
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
    fn list_cases() {
        Assert::main_binary()
            .with_args(&["list"])
            .stdout().contains("kebab-case")
            .stdout().contains("snake_case")
            .stdout().contains("UPPER CASE")
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

    // Rename related tests
    use std::path::Path;
    use std::fs;
    
    #[test] 
    fn proper_setup_cleanup() {
        setup("setup");
        assert!(Path::new("./test/tmp_setup").exists());
        cleanup("setup");
        assert!(!Path::new("./test/tmp_setup").exists());
    }

    #[test]
    fn rename_file_with_ext() {
        setup("ext");
        Assert::main_binary()
            .with_args(&["file", "test/tmp_ext/styx.txt", "-t", "pascal"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp_ext/Styx.txt").exists());
        cleanup("ext");
    }

    #[test]
    fn rename_single_file() {
        setup("single");
        Assert::main_binary()
            .with_args(&["file", "test/tmp_single/rush", "-t", "upper"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp_single/RUSH").exists());
        cleanup("single");
    }
    
    /// Copies all test data from `test/data` to `test/tmp`
    fn setup(s: &str) {
        cleanup(s);
        fs::create_dir(format!("./test/tmp_{}", s));
        for entry in fs::read_dir("./test/data").unwrap() {
            let entry = entry.unwrap();
            let original = entry.path();
            let filename = original.file_name().unwrap();
            let new = Path::new(&format!("./test/tmp_{}/file", s)).with_file_name(filename);
            fs::copy(original, new).unwrap();
        }
    }

    /// Removes all data from `test/tmp`
    fn cleanup(s: &str) {
        if Path::new(&format!("./test/tmp_{}", s)).exists() {
            for entry in fs::read_dir(format!("./test/tmp_{}", s)).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                fs::remove_file(path).unwrap();
            }
            fs::remove_dir(format!("./test/tmp_{}", s));
        }
    }
}
