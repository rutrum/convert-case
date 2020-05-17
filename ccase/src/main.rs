use atty::Stream;
use clap::{crate_version, App, AppSettings, Arg};
use convert_case::{Case, Casing};
use std::convert::TryFrom;
use std::io::{self, BufRead};

fn main() {
    let matches = create_app().get_matches();

    if matches.is_present("list-cases") {
        list_cases();
        return;
    }

    let to_case_str = matches.value_of("to-case").unwrap();
    let to_case = Case::try_from(to_case_str).unwrap();

    match matches.value_of("INPUT") {
        Some(to_convert) if !to_convert.is_empty() => {
            // User provided input inline

            match matches.value_of("from-case") {
                None => {
                    println!("{}", to_convert.to_case(to_case));
                }
                Some(from_case_str) => {
                    let from_case = Case::try_from(from_case_str).unwrap();
                    println!("{}", to_convert.from_case(from_case).to_case(to_case));
                }
            }
        }
        _ => {
            // Piped in or default empty value

            let stdin = io::stdin();
            match matches.value_of("from-case") {
                None => {
                    for line in stdin.lock().lines() {
                        let to_convert = line.expect("Unable to read from stdin");
                        println!("{}", to_convert.to_case(to_case));
                    }
                }
                Some(from_case_str) => {
                    for line in stdin.lock().lines() {
                        let to_convert = line.expect("Unable to read from stdin");
                        let from_case = Case::try_from(from_case_str).unwrap();
                        println!("{}", to_convert.from_case(from_case).to_case(to_case));
                    }
                }
            }
        }
    }
}

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
                .default_value("")
                .required_unless("list-cases")
                .requires("to-case")
                .validator(pipe_or_inline),
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
    match Case::try_from(s.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("the '{}' case is not implemented", s)),
    }
}

fn list_cases() {
    println!("Valid cases:");
    for case in Case::all_cases() {
        println!("    {:<16} {}", format!("{:?}", case), case.name_in_case());
    }
}

#[cfg(test)]
mod test {
    
    use assert_cli::Assert;

    #[test]
    fn inline_use() {
        Assert::main_binary()
            .with_args(&["-t", "snake", "myVarName"])
            .stdout().is("my_var_name")
            .unwrap();
    }

    #[test]
    fn stdin_use() {
        Assert::main_binary()
            .stdin("myVarName")
            .with_args(&["-t", "snake"])
            .stdout().is("my_var_name")
            .unwrap();
    }

    #[test]
    fn multiline_stdin() {
        Assert::main_binary()
            .stdin("one\ntwo\nthree")
            .with_args(&["-t", "title"])
            .stdout().is("One\nTwo\nThree")
            .unwrap();
    }

    #[test]
    fn bad_case() {
        Assert::main_binary()
            .with_args(&["-t", "ttle", "myVarName"])
            .fails()
            .unwrap();
        Assert::main_binary()
            .with_args(&["-t", "title", "-f", "cammel" , "myVarName"])
            .fails()
            .unwrap();
    }
}
