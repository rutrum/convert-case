use clap::{App, ArgMatches, ErrorKind};
use convert_case::{Case, Casing};
use std::io::{self, Read};

mod app;
mod list;

#[derive(Debug)]
enum Error {
    InvalidCase(String),
    InputMissing,
    ToCaseMissing,
    CaseMissing,
    Stdin,
}

impl Error {
    fn msg(&self) -> String {
        use Error::*;
        match self {
            InputMissing => "The following required arguments were not provided:\n     <INPUT>".to_string(),
            ToCaseMissing => "The following required arguments were not provided:\n     --to <CASE>".to_string(),
            CaseMissing => "The following required arguments were not provided:\n     <CASE>".to_string(),
            Stdin => "Failure to read from stdin.".to_string(),
            InvalidCase(c) => format!("Invalid value for '--to <CASE>': no such case {}", c),
        }
    }

    fn kind(&self) -> ErrorKind {
        ErrorKind::MissingRequiredArgument
    }
}

fn case_from_str(s: &str) -> Option<Case> {
    for case in Case::all_cases() {
        if format!("{:?}", case).to_case(Case::Flat) == s.to_string().to_case(Case::Flat) {
            return Some(case)
        }
    }
    None
}

fn main() -> Result<(), Error> {

    let mut app = app::create();
    let matches = app.clone().get_matches();

    match &matches.subcommand() {
        Some(("list", sub_matches)) => {
            match list_get_case(&sub_matches) {
                Ok(case) => list::print_about_case(&case),
                Err(Error::CaseMissing) => println!("{}", list::about()),
                Err(e) => return Err(e),
            }
        }
        _ => {
            resolve_no_subcommand_usage(&mut app, &matches)?;
        }
    }

    Ok(())
}

fn list_get_case(matches: &ArgMatches) -> Result<Case, Error> {
    let case_str = matches.value_of("CASE").ok_or(Error::CaseMissing)?;
    case_from_str(case_str).ok_or(Error::InvalidCase(case_str.to_string()))
}

/// Logic when no subcommand is used
fn resolve_no_subcommand_usage(app: &mut App, matches: &ArgMatches) -> Result<(), Error> {
    let input_result = get_input(&matches);
    let to_case_result = get_to_case(&matches);

    match (&input_result, &to_case_result) {
        (Err(Error::InputMissing), Err(Error::ToCaseMissing)) => {
            app.write_help(&mut io::stderr()).unwrap();
            std::process::exit(1);
        }

        (Err(e @ Error::InputMissing), _) => 
            app.error(e.kind(), e.msg()).exit(),

        (Err(e), _) => app.error(e.kind(), e.msg()).exit(),

        (_, Err(e)) => app.error(e.kind(), e.msg()).exit(),

        _ => {}
    }

    let input = input_result.unwrap();
    let to_case = to_case_result.unwrap();

    if let Some(from_case_str) = matches.value_of("from-case") {
        let from_case = case_from_str(from_case_str).ok_or(Error::InvalidCase(from_case_str.to_string()))?;
        for line in input.split("\n") {
            let converted = line.from_case(from_case).to_case(to_case);
            println!("{}", converted);
        }
    } else {
        for line in input.split("\n") {
            let converted = line.to_case(to_case);
            println!("{}", converted);
        }
    };

    Ok(())
}

fn get_to_case<'a>(matches: &'a ArgMatches) -> Result<Case, Error> {
    let to_case_str = matches.value_of("to-case").ok_or(Error::ToCaseMissing)?;
    case_from_str(to_case_str).ok_or(Error::InvalidCase(to_case_str.to_string()))
}

/// This should really return a buffer, not a string, then run the command on each line
fn get_input<'a>(matches: &'a ArgMatches) -> Result<String, Error> {
    if let Some(input) = matches.value_of("INPUT") {
        return Ok(input.into());
    } 

    if atty::isnt(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut v = Vec::new();
        handle.read_to_end(&mut v).map_err(|_| Error::Stdin)?;

        let s = String::from_utf8(v)
            .map_err(|_| Error::Stdin)?
            .to_string();

        if s.is_empty() {
            Err(Error::InputMissing)
        } else {
            Ok(s)
        }
    } else {
        Err(Error::InputMissing)
    }
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn to_case() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["-t", "snake", "myVarName"])
            .assert()
            .success()
            .stdout("my_var_name\n");

        Command::cargo_bin("ccase").unwrap()
            .args(&["myVarName", "--to", "kebab"])
            .assert()
            .success()
            .stdout("my-var-name\n");
    }

    #[test]
    fn from_case() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["-f", "camel", "-t", "snake", "myVar-Name"])
            .assert()
            .success()
            .stdout("my_var-name\n");

        Command::cargo_bin("ccase").unwrap()
            .args(&["-t", "camel", "--from", "kebab", "my-Var-Name_longer"])
            .assert()
            .success()
            .stdout("myVarName_longer\n");
    }

    #[test]
    fn invalid_case() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["-t", "blah", "myVarName"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: Invalid value for '--to <CASE>'"));

        Command::cargo_bin("ccase").unwrap()
            .args(&["-f", "blah", "-t", "snake", "string"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: Invalid value for '--from <CASE>'"));
    }

    #[test]
    fn no_to_case() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["myVarName"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: The following required arguments were not provided").and(
                    predicate::str::contains("--to <CASE>")
            ));

        Command::cargo_bin("ccase").unwrap()
            .args(&["-f", "kebab", "myVarName"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: The following required arguments were not provided").and(
                    predicate::str::contains("--to <CASE>")
            ));
    }

    #[test]
    fn no_to_case_stdin() {
        Command::cargo_bin("ccase").unwrap()
            .write_stdin("varName")
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: The following required arguments were not provided").and(
                    predicate::str::contains("--to <CASE>")
            ));

        Command::cargo_bin("ccase").unwrap()
            .write_stdin("varName")
            .args(&["-f", "kebab"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: The following required arguments were not provided").and(
                    predicate::str::contains("--to <CASE>")
            ));
    }

    #[test]
    fn input_from_stdin() {
        Command::cargo_bin("ccase").unwrap()
            .write_stdin("myVarName")
            .args(&["-t", "snake"])
            .assert()
            .success()
            .stdout("my_var_name\n");
    }

    #[test]
    fn no_input() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["-t", "snake"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("error: The following required arguments were not provided").and(
                    predicate::str::contains("<INPUT>")
            ));
    }

    #[test]
    fn multiline_input() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["-t", "pascal"])
            .write_stdin("cat_dog\ndog_cat")
            .assert()
            .success()
            .stdout("CatDog\nDogCat\n");

        Command::cargo_bin("ccase").unwrap()
            .args(&["-f", "snake", "-t", "pascal"])
            .write_stdin("cat_dog\ndog_cat")
            .assert()
            .success()
            .stdout("CatDog\nDogCat\n");
    }

    #[test]
    fn no_arg_show_help() {
        Command::cargo_bin("ccase").unwrap()
            .assert()
            .failure()
            .stderr(predicate::str::contains("USAGE").and(
                    predicate::str::contains("ARGS").and(
                    predicate::str::contains("OPTIONS")
            )));
    }

    #[test]
    fn bad_ordering_to_from() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["myVar-Name", "-t"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("The argument '--to <CASE>' requires a value"));

        Command::cargo_bin("ccase").unwrap()
            .args(&["-f", "-t", "kebab", "blah"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("The argument '--from <CASE>' requires a value"));
    }

    #[test]
    fn to_case_not_lower() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["--to", "KeBAB", "myVarName"])
            .assert()
            .success()
            .stdout("my-var-name\n");

        Command::cargo_bin("ccase").unwrap()
            .args(&["--from", "KeBAB", "-t", "snake", "my-bad-VARiable"])
            .assert()
            .success()
            .stdout("my_bad_variable\n");
    }

    #[test]
    fn newlines_as_stdin() {
        Command::cargo_bin("ccase").unwrap()
            .write_stdin("\n\n")
            .args(&["-t", "snake"])
            .assert()
            .success()
            .stdout("\n\n\n");
    }

    #[test]
    fn empty_string_as_arg() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["-t", "snake", r#""#])
            .assert()
            .success()
            .stdout("\n");
    }
}
