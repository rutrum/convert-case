use atty::Stream;
use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use convert_case::Case;

use ccase_lib::conversion::Conversion;
use ccase_lib::CaseClassification;
use ccase_lib::Error;

// Idea: add option -d aA0 to split on lower to upper, and upper to digit

fn main() -> Result<(), Error> {
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

    Ok(())
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Convert Case")
        .version(crate_version!())
        .author("Dave Purdum <purdum41@gmail.com>")
        .about("Converts to and from various cases.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
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
                .args(&case_args()),
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

fn case_args() -> Vec<Arg<'static, 'static>> {
    vec![
        Arg::with_name("to-case")
            .short("t")
            .long("to")
            .value_name("CASE")
            .help("The case to convert into.")
            .takes_value(true)
            .validator(is_valid_case),
        Arg::with_name("from-case")
            .short("f")
            .long("from")
            .value_name("CASE")
            .help("The case to parse input as.")
            .validator(is_valid_case)
            .takes_value(true),
    ]
}

// Returns true if either a string is provided or data
// is being piped in from stdin
fn pipe_or_inline(s: String) -> Result<(), String> {
    let valid = !s.is_empty() || atty::isnt(Stream::Stdin);
    if valid {
        Ok(())
    } else {
        Err("require input inline or from stdin".to_string())
    }
}

fn is_valid_case(s: String) -> Result<(), String> {
    match Case::from_str(s.as_str()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("the '{}' case is not implemented", s)),
    }
}

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

    // Rename related tests
    use std::fs;
    use std::path::Path;

    #[test]
    fn proper_setup_cleanup() {
        setup("setup");
        assert!(Path::new("./test/tmp/setup").exists());
        cleanup("setup");
        assert!(!Path::new("./test/tmp/setup").exists());
    }

    #[test]
    fn rename_file_with_ext() {
        setup("ext");
        Assert::main_binary()
            .with_args(&["file", "test/tmp/ext/styx.txt", "-e", "-t", "pascal"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp/ext/Styx.Txt").exists());
        Assert::main_binary()
            .with_args(&["file", "test/tmp/ext/van_halen.exe", "-e", "-t", "UPPER"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp/ext/VAN HALEN.EXE").exists());
        cleanup("ext");
    }

    #[test]
    fn rename_file_and_ext() {
        setup("file_ext");
        Assert::main_binary()
            .with_args(&["file", "test/tmp/file_ext/styx.txt", "-t", "pascal"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp/file_ext/Styx.txt").exists());
        cleanup("file_ext");
    }

    #[test]
    fn rename_single_file() {
        setup("single");
        Assert::main_binary()
            .with_args(&["file", "test/tmp/single/rush", "-t", "upper"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp/single/RUSH").exists());
        cleanup("single");
    }

    #[test]
    fn rename_multiple_dots() {
        setup("dots");
        Assert::main_binary()
            .with_args(&["file", "test/tmp/dots/blue-oyster.cult.gdz", "-t", "title"])
            .succeeds()
            .unwrap();
        assert!(Path::new("./test/tmp/dots/Blue Oyster.cult.gdz").exists());
        cleanup("dots");
    }

    /// Copies all test data from `test/data` to `test/tmp`
    fn setup(s: &str) {
        cleanup(s);
        if !Path::new("./test/tmp").exists() {
            fs::create_dir("./test/tmp").unwrap();
        }
        fs::create_dir(format!("./test/tmp/{}", s)).ok();
        for entry in fs::read_dir("./test/data").unwrap() {
            let entry = entry.unwrap();
            let original = entry.path();
            let filename = original.file_name().unwrap();
            let new = Path::new(&format!("./test/tmp/{}/file", s)).with_file_name(filename);
            fs::copy(original, new).unwrap();
        }
    }

    /// Removes all data from `test/tmp`
    fn cleanup(s: &str) {
        if Path::new(&format!("./test/tmp/{}", s)).exists() {
            for entry in fs::read_dir(format!("./test/tmp/{}", s)).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                fs::remove_file(path).unwrap();
            }
            fs::remove_dir(format!("./test/tmp/{}", s)).ok();
        }
    }
}
