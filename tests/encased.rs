#![cfg(feature = "encased")]

use convert_case::{Case, Casing};
#[test]
fn encased_string_type() {
    let s: String = String::from("rust_programming_language");
    assert_eq!(
        "RustProgrammingLanguage",
        s.encased::<{ Case::Pascal }>().raw(),
    );
}

#[test]
fn encased_str_type() {
    let s: &str = "rust_programming_language";
    assert_eq!(
        "RustProgrammingLanguage",
        s.encased::<{ Case::Pascal }>().raw(),
    );
}

#[test]
fn encased_string_ref_type() {
    let s: String = String::from("rust_programming_language");
    assert_eq!(
        "RustProgrammingLanguage",
        (&s).encased::<{ Case::Pascal }>().raw(),
    );
}
