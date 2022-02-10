//! Functions for creating the clap cli application

use clap::{App, Arg, ColorChoice, crate_version};
use convert_case::{Case, Casing};

pub fn create<'a>() -> App<'a> {
    App::new("ccase")
        .version(crate_version!())
        .author("Dave Purdum <davepurdum@pm.me>")
        .about("Converts strings to and from cases.")
        .color(ColorChoice::Never)
        .args(vec![
            arg_input(),
            arg_to_case(),
            arg_from_case(),
        ])
}

/*
fn subcommand_list<'a>() -> App<'a> {
    App::new("")
}
*/

fn arg_input<'a>() -> Arg<'a> {
    Arg::new("INPUT")
        .help("String to convert.")
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
