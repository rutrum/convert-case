//! Functions for creating the clap cli application

use clap::{App, AppSettings, Arg, ColorChoice, crate_version, crate_authors};
use convert_case::{Case, Casing};
use crate::list;

pub fn create<'a>() -> App<'a> {
    App::new("ccase")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Converts strings to and from cases.")
        .color(ColorChoice::Never)
        .args(vec![
            arg_input(),
            arg_to_case(),
            arg_from_case(),
        ])
        .subcommand(subcommand_list())
}

fn subcommand_list<'a>() -> App<'a> {
    App::new("list")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Description of string cases.")
        .color(ColorChoice::Never)
        .arg(arg_case())
}

fn arg_case<'a>() -> Arg<'a> {
    Arg::new("CASE")
        .help("Case to query.")
}

fn arg_input<'a>() -> Arg<'a> {
    Arg::new("INPUT")
        .help("String to convert.")
}

fn arg_boundaries<'a>() -> Arg<'a> {
    Arg::new("boundaries")
        .short('b')
        .value_name("BOUNDARY_STRING")
        .help("String of boundaries to split by.")
        .takes_value(true)
}

fn matches_case(s: &str) -> Result<(), String> {
    for case in Case::all_cases() {
        if format!("{:?}", case).to_case(Case::Flat) == s.to_string().to_case(Case::Flat) {
            return Ok(());
        }
    }
    Err(format!("no such case `{}`", s))
}

fn arg_to_case<'a>() -> Arg<'a> {
    Arg::new("to-case")
        .short('t')
        .long("to")
        .value_name("CASE")
        .help("Case to convert string into.")
        .takes_value(true)
        .validator(|s| matches_case(s))
}

fn arg_from_case<'a>() -> Arg<'a> {
    Arg::new("from-case")
        .short('f')
        .long("from")
        .value_name("CASE")
        .help("Case to convert string from.")
        .takes_value(true)
        .validator(matches_case)
}
