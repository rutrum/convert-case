//! Functions for creating the clap cli application

use clap::{App, AppSettings, Arg, crate_version};

pub fn create<'a>() -> App<'a> {
    App::new("Convert Case")
        .version(crate_version!())
        .author("Dave Purdum <davepurdum@pm.me>")
        .about("Converts strings to and from cases.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .args(vec![
            arg_input(),
            arg_to_case(),
            arg_from_case(),
        ])
}

fn arg_input<'a>() -> Arg<'a> {
    Arg::new("INPUT")
        .help("String to convert.")
        .default_value("")
        .validator(validate_input)
}

fn validate_input(s: &str) -> Result<(), String> {
    if s.trim().len() > 0 || atty::isnt(atty::Stream::Stdin) {
        Ok(())
    } else {
        Err("required value from stdin or as an argument".to_string())
    }
}

fn arg_to_case<'a>() -> Arg<'a> {
    Arg::new("to-case")
        .short('t')
        .long("to")
        .value_name("CASE")
        .help("Case to convert string into.")
        .takes_value(true)
        .required(true)
}

fn arg_from_case<'a>() -> Arg<'a> {
    Arg::new("from-case")
        .short('f')
        .long("from")
        .value_name("CASE")
        .help("Case to convert string from.")
        .takes_value(true)
}
