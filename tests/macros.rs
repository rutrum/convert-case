use convert_case::ccase;
use convert_case::{separator, split};

#[test]
fn ccase_to_case() {
    assert_eq!(ccase!(snake, "my_Var_Name"), "my_var_name");
    assert_eq!(ccase!(constant, "my_Var_Name"), "MY_VAR_NAME");
    assert_eq!(ccase!(kebab, "my_Var_Name"), "my-var-name");
    assert_eq!(ccase!(kebab, String::from("my_Var_Name")), "my-var-name");
}

#[test]
fn ccase_from_to_case() {
    assert_eq!(ccase!(kebab -> camel, "myVar-name_var"), "myvarName_var");
    assert_eq!(ccase!(snake -> pascal, "my-var_name-var"), "My-varName-var");
}

#[test]
fn separator_custom_delimiters() {
    let dot = separator!(".");
    assert_eq!(
        split(&"lower.Upper.Upper", &[dot]),
        vec!["lower", "Upper", "Upper"]
    );

    let double_colon = separator!("::");
    assert_eq!(
        split(&"lower::lowerUpper::Upper", &[double_colon]),
        vec!["lower", "lowerUpper", "Upper"]
    );
}

#[test]
fn separator_non_ascii_delimiters() {
    // A multi-byte delimiter can be a single grapheme (an arrow is 3 bytes,
    // 1 grapheme) or span several graphemes (two arrows: 6 bytes, 2
    // graphemes). separator! must not confuse the delimiter byte length with
    // its grapheme count.
    let arrow = separator!("→");
    assert_eq!(split(&"lower→Upper", &[arrow]), vec!["lower", "Upper"]);

    let kanji_dot = separator!("。");
    assert_eq!(split(&"東。京", &[kanji_dot]), vec!["東", "京"]);

    let arrows = separator!("→→");
    assert_eq!(split(&"lower→→Upper", &[arrows]), vec!["lower", "Upper"]);
}
