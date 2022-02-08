//! For the `file` subcommand

use clap::{App, crate_version, AppSettings, Arg, SubCommand};

use crate::pipe_or_inline;
use crate::case_args;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
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
        .arg(
            Arg::with_name("ext")
                .short("e")
                .long("ext")
                .help("Use to also convert the file extension.")
                .long_help(
                    "Will convert the file extension as though \
                           it were separate identifier, in addition to the filename.",
                ),
        )
        .args(&case_args())
}
