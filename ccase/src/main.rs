use clap::{ArgMatches};
use convert_case::{Case, Casing};
use std::io::{self, Read};

mod app;

#[derive(Debug)]
enum Error {
    NoToCase,
    NoSuchCase,
    Stdin,
    NoInput,
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
    let app = app::create();
    let matches = app.get_matches();

    let to_case_str = matches.value_of("to-case").ok_or(Error::NoToCase)?;
    let to_case = case_from_str(to_case_str).ok_or(Error::NoSuchCase)?;

    let input = get_input(&matches)?;

    if let Some(from_case_str) = matches.value_of("from-case") {
        let from_case = case_from_str(from_case_str).ok_or(Error::NoSuchCase)?;
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
        Err(Error::NoInput)
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
    fn input_from_stdin() {
        Assert::main_binary()
            .with_args(&["-t", "snake"])
            .stdin("myVarName")
            .stdout()
            .is("my_var_name")
            .unwrap();
    }

    #[test]
    #[ignore] // Doesn't work automatically, can verify manually
    fn no_input() {
        Assert::main_binary()
            .with_args(&["-t", "snake"])
            .fails()
            .stderr()
            .contains("error")
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
