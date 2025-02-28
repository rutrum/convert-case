use convert_case::ccase;

#[test]
fn ccase_snake() {
    assert_eq!("my_var_name", ccase!(snake, "my_Var_Name"));
}

#[test]
fn ccase_constant() {
    assert_eq!("MY_VAR_NAME", ccase!(constant, "my_Var_Name"));
}

#[test]
fn ccase_kebab() {
    assert_eq!("my-var-name", ccase!(kebab, "my_Var_Name"));
}

#[test]
fn ccase_kebab_string() {
    assert_eq!("my-var-name", ccase!(kebab, String::from("my_Var_Name")));
}

#[test]
fn ccase_from_kebab_to_camel() {
    assert_eq!("myvarName_var", ccase!(kebab, camel, "myVar-name_var"));
    assert_eq!("myvarName_var", ccase!(kebab -> camel, "myVar-name_var"));
}
/*
#[test]
fn ccase_random() {
    assert_ne!("my-var-name", ccase!(random, "my_Var_Name"))
}
*/

use convert_case::{AsciiCase, Case, Casing};

fn main() {
    assert_eq!("my-var", "my_var".to_case(Case::Kebab));

    assert_eq!(
        "Registration Names 2024 10 22",
        "registration_names_2024-10-22".to_case(Case::Title)
    );

    assert_eq!(
        "Registration Names 2024-10-22",
        "registration_names_2024-10-22"
            .from_case(Case::Snake)
            .to_case(Case::Title)
    );

    let s: &str = "my_var_name";
    let ascii_s: &ascii_str = AsciiStr::try_new(s).unwrap();
    ascii_s.to_case(Case::Camel);

    assert_eq!(
        "Registration Names 2024-10-22",
        "registration_names_2024-10-22"
            .from_ascii_case(Case::Snake)
            .to_ascii_case(Case::Title)
    );
    // to_ascii_lowercase still parses as unicode, just ignores non-ascii characters
}
