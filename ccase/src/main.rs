use clap::{App, AppSettings, Arg};
//use std::io::{self, BufRead};
use convert_case::{Case, Casing};
use std::convert::TryFrom;

fn main() {
    let matches = App::new("Convert Case")
        .version("0.1")
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
    /*
    match matches.value_of("from-case") {
        Some(from_case_str) => {
            let from_case = Case::try_from(from_case_str).unwrap();
            run_conversion(matches, |s: &str| {
                s.from_case(from_case).to_case(to_case)
            });
        },
        None => {
            run_conversion(matches, |s: &str| s.to_case(to_case));
        },
    };
    */
}
/*
// Either reads input from argument or stdin
fn run_conversion<F>(matches: ArgMatches, convert: F)
    where F: Fn(&str) -> String
{
    match matches.value_of("INPUT") {
        Some(to_convert) => {
            println!("{}", convert(to_convert));
        },
        None => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                println!("{}", convert(&line.expect("Unable to read from stdin")));
            }
        }
    }
}
*/

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
