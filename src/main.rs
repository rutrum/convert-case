use clap::{App, Arg};
use convert_case::{Case, Casing};
use strum::IntoEnumIterator;
use std::convert::TryFrom;

fn main() {
    let matches = App::new("Convert Case")
        .version("0.1")
        .author("Dave Purdum <purdum41@gmail.com>")
        .about("Converts to and from various cases.")
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
                .required_unless("list-cases")
                .requires("to-case"),
        )
        .get_matches();

    if matches.is_present("list-cases") {
        list_cases();
        return;
    }

    let to_case_str = matches.value_of("to-case").unwrap();
    let to_case = Case::try_from(to_case_str).unwrap();

    let to_convert = matches.value_of("INPUT").unwrap();

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

fn is_valid_case(s: String) -> Result<(), String> {
    match Case::try_from(s.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("the '{}' case is not implemented", s)),
    }
}

fn list_cases() {
    println!("Valid cases:");
    for case in Case::iter() {
        println!("{:>15}  {}", format!("{:?}", case), case.example());
    }
}

trait Example {
    fn example(&self) -> String;
}

impl Example for Case {
    fn example(&self) -> String {
        use Case::*;
        match self {
            Title => "My Variable Name",
            Camel => "myVariableName",
            Pascal | UpperCamel => "MyVariableName",
            Snake => "my_variable_name",
            Kebab => "my-variable-name",
            UpperSnake | ScreamingSnake => "MY_VARIABLE_NAME",
            Upper => "MY VARIABLE NAME",
            Lower => "my variable name",
            Cobol => "MY-VARIABLE-NAME",
            Flat => "myvariablename",
            Toggle => "mY vARIABLE nAME",
            Train => "My-Variable-Name",
            UpperFlat => "MYVARIABLENAME",
        }
        .to_string()
    }
}
