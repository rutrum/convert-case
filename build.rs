use std::{env, fs, path::Path};

fn main() {
    let encased = env::var_os("CARGO_FEATURE_ENCASED").is_some();
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = Path::new(&out_dir).join("src/lib.rs");

    let content = fs::read_to_string(lib_path.clone()).unwrap();

    let new_content = if encased {
        content.replace(
            "//ADT_CONST_PARAMS_REPLACE",
            "#![feature(adt_const_params)]",
        )
    } else {
        content.replace(
            "#![feature(adt_const_params)]",
            "//ADT_CONST_PARAMS_REPLACE",
        )
    };

    fs::write(&lib_path, new_content).unwrap();
}
