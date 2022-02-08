use crate::{CaseClassification, Error};
use clap::ArgMatches;
use convert_case::{Case, Casing};
use std::io::{self, BufRead};
use std::path::Path;

pub struct Conversion {
    pub to: Case,
    pub from: Option<Case>,
    pub strings: Vec<String>,
    pub converted: Vec<String>,
}

impl Conversion {
    pub fn strings(matches: &ArgMatches) -> Result<Self, Error> {
        let to = Self::to_from_matches(&matches)?;
        let from = Self::from_from_matches(&matches)?;
        let strings = Self::input_from_matches_or_stdin(&matches, "INPUT")?;

        let converted = strings
            .iter()
            .map(|s| match from {
                Some(from) => s.from_case(from).to_case(to),
                None => s.to_case(to),
            })
            .collect();

        Ok(Conversion {
            to,
            from,
            strings,
            converted,
        })
    }

    pub fn paths(matches: &ArgMatches) -> Result<Self, Error> {
        let to = Self::to_from_matches(&matches)?;
        let from = Self::from_from_matches(&matches)?;
        let strings = Self::input_from_matches_or_stdin(&matches, "PATH")?;
        let include_ext = matches.is_present("ext");

        let converted = strings
            .iter()
            .map(|s| {
                let p = Path::new(s);
                let filename = p
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();
                println!("{}", filename);
                let new_filename = match from {
                    Some(from) => filename.from_case(from).to_case(to),
                    None => filename.to_case(to),
                };
                match p.extension() {
                    Some(e) => {
                        let mut ext = e.to_owned().into_string().unwrap();
                        if include_ext {
                            ext = match from {
                                Some(from) => ext.from_case(from).to_case(to),
                                None => ext.to_case(to),
                            }
                        }
                        match p.parent() {
                            Some(p) => {
                                let mut p = p.join(format!("{}.tmp", new_filename)).to_path_buf();
                                p.set_extension(ext);
                                format!("{}", p.into_os_string().into_string().unwrap())
                            }
                            None => {
                                let mut p =
                                    Path::new(&format!("{}.tmp", new_filename)).to_path_buf();
                                p.set_extension(ext);
                                format!("{}", p.into_os_string().into_string().unwrap())
                            }
                        }
                    }
                    None => match p.parent() {
                        Some(p) => {
                            format!(
                                "{}",
                                p.join(new_filename).into_os_string().into_string().unwrap()
                            )
                        }
                        None => new_filename,
                    },
                }
            })
            .collect();

        Ok(Conversion {
            to,
            from,
            strings,
            converted,
        })
    }

    fn to_from_matches(matches: &ArgMatches) -> Result<Case, Error> {
        let to_str = matches.value_of("to-case").ok_or(Error::NoToCase)?;
        let to = Case::from_str(to_str).map_err(|_| Error::InvalidCase(to_str.to_string()))?;
        Ok(to)
    }

    fn from_from_matches(matches: &ArgMatches) -> Result<Option<Case>, Error> {
        let from = match matches.value_of("from-case") {
            None => None,
            Some(from_str) => Some(
                Case::from_str(from_str).map_err(|_| Error::InvalidCase(from_str.to_string()))?,
            ),
        };
        Ok(from)
    }

    fn input_from_matches_or_stdin(
        matches: &ArgMatches,
        input: &'static str,
    ) -> Result<Vec<String>, Error> {
        let strings = match matches.value_of(input) {
            Some(s) if !s.is_empty() => vec![s.to_string()],
            _ => {
                let mut lines = vec![];
                for line in io::stdin().lock().lines() {
                    lines.push(line.map_err(|_| Error::Stdin)?);
                }
                lines
            }
        };
        Ok(strings)
    }
}
