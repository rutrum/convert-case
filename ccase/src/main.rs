use clap::{ArgMatches, ErrorKind};
use convert_case::{Case, Casing};
use std::io::{self, Read};

mod app;

#[derive(Debug)]
enum Error {
    InvalidCase,
    InputMissing,
    ToCaseMissing,
    Stdin,
}

impl Error {
    fn msg(&self) -> &str {
        use Error::*;
        match self {
            InputMissing => "The following required arguments were not provided:\n     <INPUT>",
            ToCaseMissing => "The following required arguments were not provided:\n     --to <CASE>",
            Stdin => "Failure to read from stdin",
            _ => ""
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

    let input_result = get_input(&matches);
    let to_case_result = get_to_case(&matches);

    match (&input_result, &to_case_result) {
        (Err(Error::InputMissing), Err(Error::ToCaseMissing)) => {
            app.write_help(&mut io::stderr()).unwrap();
            return Ok(())
        }

        (Err(e @ Error::InputMissing), _) => 
            app.error(e.kind(), e.msg()).exit(),

        (Err(e), _) => app.error(e.kind(), e.msg()).exit(),

        (_, Err(e)) => app.error(e.kind(), e.msg()).exit(),

        _ => {}
    }

    /*
    if let Err(ref e) = input {
        if matches.value_of("to-case").is_some() {
            app.error(
                ErrorKind::MissingRequiredArgument,
                e.msg()
            ).exit()
        } else {
            app.print_help().unwrap();
            return Ok(())
        }
    }
    */

    let input = input_result.unwrap();
    let to_case = to_case_result.unwrap();

    if let Some(from_case_str) = matches.value_of("from-case") {
        let from_case = case_from_str(from_case_str).ok_or(Error::InvalidCase)?;
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
    case_from_str(to_case_str).ok_or(Error::InvalidCase)
}

fn get_input<'a>(matches: &'a ArgMatches) -> Result<String, Error> {
    if let Some(input) = matches.value_of("INPUT") {
        if input.trim().len() > 0 {
            return Ok(input.into());
        }
    } 

    if atty::isnt(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut v = Vec::new();
        handle.read_to_end(&mut v).map_err(|_| Error::Stdin)?;

        let s = String::from_utf8(v).map_err(|_| Error::Stdin)?;
        Ok(s.trim().to_string())
    } else {
        Err(Error::InputMissing)
    }
}

#[cfg(test)]
mod test {
    use assert_cli::Assert;

    #[test]
    fn to_case() {
        Assert::main_binary()
            .with_args(&["-t", "snake", "myVarName"])
            .stdout()
            .is("my_var_name")
            .unwrap();

        Assert::main_binary()
            .with_args(&["myVarName", "--to", "kebab"])
            .stdout()
            .is("my-var-name")
            .unwrap();
    }

    #[test]
    fn invalid_case() {
        Assert::main_binary()
            .with_args(&["-t", "blah", "asdfASDF"])
            .fails()
            .stderr()
            .contains("error: Invalid value for '--to <CASE>'")
            .unwrap();

        Assert::main_binary()
            .with_args(&["-f", "blah", "-t", "snake", "asdfASDF"])
            .fails()
            .stderr()
            .contains("error: Invalid value for '--from <CASE>'")
            .unwrap();
    }

    #[test]
    fn no_to_case_input_argument() {
        Assert::main_binary()
            .with_args(&["varName"])
            .fails()
            .stderr()
            .contains("error: The following required arguments were not provided")
            .stderr()
            .contains("--to <CASE>")
            .unwrap();
    }

    #[test]
    fn no_to_case_input_stdin() {
        Assert::main_binary()
            .stdin("varName")
            .fails()
            .stderr()
            .contains("error: The following required arguments were not provided")
            .stderr()
            .contains("--to <CASE>")
            .unwrap();
    }

    #[test]
    fn input_from_stdin() {
        Assert::main_binary()
            .with_args(&["-t", "snake"])
            .stdin("myVarName")
            .stdout()
            .is("my_var_name")
            .unwrap();
    }

    #[test]
    fn no_input() {
        Assert::main_binary()
            .with_args(&["-t", "snake"])
            .fails()
            .stderr()
            .contains("error: The following required arguments were not provided")
            .stderr()
            .contains("<INPUT>")
            .unwrap();
    }

    #[test]
    fn multiline_input() {
        Assert::main_binary()
            .with_args(&["-t", "pascal"])
            .stdin("cat_dog\ndog_cat")
            .stdout()
            .contains("CatDog\nDogCat")
            .unwrap();

        Assert::main_binary()
            .with_args(&["-f", "snake", "-t", "pascal"])
            .stdin("cat_dog\ndog_cat")
            .stdout()
            .contains("CatDog\nDogCat")
            .unwrap();
    }

    #[test]
    fn default_shows_help() {
        Assert::main_binary()
            .fails()
            .stderr()
            .contains("USAGE")
            .stderr()
            .contains("ARGS")
            .stderr()
            .contains("OPTIONS")
            .unwrap();
    }

    #[test]
    fn bad_to_from_input() {
        // No to value
        Assert::main_binary()
            .with_args(&["myVar-Name", "-t"])
            .fails()
            .unwrap();

        // No from value
        Assert::main_binary()
            .with_args(&["-f", "-t", "kebab", "blah"])
            .fails()
            .unwrap();
    }

    #[test]
    fn from_case() {
        Assert::main_binary()
            .with_args(&["-f", "camel", "-t", "snake", "myVar-Name"])
            .stdout()
            .is("my_var-name")
            .unwrap();

        Assert::main_binary()
            .with_args(&["-t", "camel", "--from", "kebab", "my-Var-Name_longer"])
            .stdout()
            .is("myVarName_longer")
            .unwrap();
    }

    #[test]
    fn to_case_not_lower() {
        Assert::main_binary()
            .with_args(&["--to", "KeBAB", "myVarName"])
            .stdout()
            .is("my-var-name")
            .unwrap();

        Assert::main_binary()
            .with_args(&["--from", "KeBAB", "-t", "snake", "my-bad-VARiable"])
            .stdout()
            .is("my_bad_variable")
            .unwrap();
    }

}
