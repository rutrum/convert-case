use convert_case::ccase;

#[test]
fn ccase_snake() {
    assert_eq!("my_var_name", ccase!(snake, "my_Var_Name"))
}

#[test]
fn ccase_kebab() {
    assert_eq!("my-var-name", ccase!(kebab, "my_Var_Name"))
}
