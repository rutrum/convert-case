use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use convert_case::Case;

use ccase_lib::conversion::Conversion;
use ccase_lib::CaseClassification;
use ccase_lib::Error;

use ccase_lib::file;
use ccase_lib::pipe_or_inline;
use ccase_lib::case_args;

// Idea: add option -d aA0 to split on lower to upper, and upper to digit
//
// examples
//
// ccase --to snake -d aA myVarName
// ccase list --help
// ccase list snake
// ccase

fn main() -> Result<(), Error> {
    /*
    let app = create_app();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("list", _) => {
            Case::list();
            return Ok(());
        }
        ("file", Some(sub_m)) => {
            let conversion = Conversion::paths(&sub_m)?;
            for (o, n) in conversion.strings.iter().zip(conversion.converted.iter()) {
                match std::fs::rename(o, n) {
                    Ok(_) => println!("{} => {}", o, n),
                    Err(e) => {
                        println!("{}", e);
                        return Ok(());
                    }
                }
            }
        }
        _ => {
            // No special command, just convert input
            let conversion = Conversion::strings(&matches)?;
            for s in conversion.converted {
                println!("{}", s);
            }
        }
    }
    */

    Ok(())
}

/*
fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Convert Case")
        .version(crate_version!())
        .author("Dave Purdum <purdum41@gmail.com>")
        .about("Converts to and from various cases.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            file::subcommand()
        )
        .subcommand(SubCommand::with_name("list").about("List available cases"))
        .arg(
            Arg::with_name("INPUT")
                .help("The string to convert.")
                .requires("to-case")
                .validator(pipe_or_inline),
        )
        .args(&case_args())
}
*/

#[cfg(test)]
mod test {
    use assert_cli::Assert;

    #[test]
    fn inline_use() {
        Assert::main_binary()
            .with_args(&["-t", "snake", "myVarName"])
            .stdout()
            .is("my_var_name")
            .unwrap();
    }

    #[test]
    fn stdin_use() {
        Assert::main_binary()
            .stdin("myVarName")
            .with_args(&["-t", "snake"])
            .stdout()
            .is("my_var_name")
            .unwrap();
    }

    #[test]
    fn list_cases() {
        Assert::main_binary()
            .with_args(&["list"])
            .stdout()
            .contains("kebab-case")
            .stdout()
            .contains("snake_case")
            .stdout()
            .contains("UPPER CASE")
            .unwrap();
    }

    #[test]
    fn help_by_default() {
        Assert::main_binary()
            .fails()
            .stderr()
            .contains("FLAGS:")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn prints_require_input() {
        Assert::main_binary()
            .with_args(&["-t", "snake"])
            .fails()
            .stderr()
            .contains("The following required arguments were not provided")
            .unwrap();
    }

    #[test]
    fn multiline_stdin() {
        Assert::main_binary()
            .stdin("one\ntwo\nthree")
            .with_args(&["-t", "title"])
            .stdout()
            .is("One\nTwo\nThree")
            .unwrap();
    }

    #[test]
    fn bad_case() {
        Assert::main_binary()
            .with_args(&["-t", "ttle", "myVarName"])
            .fails()
            .unwrap();
        Assert::main_binary()
            .with_args(&["-t", "title", "-f", "cammel", "myVarName"])
            .fails()
            .unwrap();
    }
}
