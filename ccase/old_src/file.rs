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

#[cfg(test)]
mod test {
    use assert_cli::Assert;
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
