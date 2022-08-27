//! Methods related to classifying and documenting cases

use convert_case::{Boundary, Case, Casing, Converter, Pattern};

pub fn print_about_case(case: &Case) {
    println!("{}\n\n{:>10}: {}\n{:>10}: {}\n{:>10}: {}",
        case_in_case(&case),
        "pattern",
        pattern_in_pattern(&case.pattern()),
        "delimeter",
        case.delim(),
        "boundaries",
        case.boundaries()
            .iter()
            .map(|b| format!("{:?} ({})", b, boundary_shortcode(b)))
            .collect::<Vec<String>>()
            .join("\n            "),
    )
}

pub fn case_in_case(case: &Case) -> String {
    format!("{:?} case", case).to_case(*case)
}

pub fn pattern_in_pattern(pattern: &Pattern) -> String {
    let conv = Converter::new()
        .set_pattern(*pattern);
    conv.convert(format!("{:?}", pattern))
}

pub fn boundary_shortcode(boundary: &Boundary) -> &'static str {
    use Boundary::*;
    match boundary {
        Hyphen => "-",
        Underscore => "_",
        Space => " ",
        UpperLower => "Aa",
        LowerUpper => "aA",
        DigitUpper => "1A",
        UpperDigit => "A1",
        DigitLower => "1a",
        LowerDigit => "a1",
        Acronym => "AAa",
    }
}

#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use predicates::prelude::*;

    #[test]
    fn shows_pattern() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["list", "snake"])
            .assert()
            .success()
            .stdout(predicate::str::contains("pattern").and(
                    predicate::str::contains("lowercase")
            ));
    }

    #[test]
    fn shows_boundaries() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["list", "camel"])
            .assert()
            .success()
            .stdout(predicate::str::contains("boundaries").and(
                    predicate::str::contains("LowerUpper").and(
                    predicate::str::contains("LowerDigit").and(
                    predicate::str::contains("DigitUpper")
            ))));
    }

    #[test]
    fn help_by_default() {
        Command::cargo_bin("ccase").unwrap()
            .args(&["list"])
            .assert()
            .failure()
            .stderr(predicate::str::contains("ccase-list").and(
                    predicate::str::contains("USAGE").and(
                    predicate::str::contains("ARGS").and(
                    predicate::str::contains("OPTIONS")
            ))));
    }
}
