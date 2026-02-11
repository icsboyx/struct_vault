use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, load, load_or_default};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TestStruct {
    pub name: String,
    pub value: i32,
}

fn main() {
    let file = "not_existing_test_struct";
    let format = SaveFormat::Toml;

    println!("=================================================");
    println!("load<T>() – expected failure (file missing)");
    println!("file   : {file}");
    println!("format : TOML");
    println!("-------------------------------------------------");

    match load::<TestStruct>(file, None, format) {
        Ok(v) => {
            println!("OK (unexpected)");
            println!("{v:#?}");
        }
        Err(e) => {
            println!("ERROR (expected)");
            println!("{e:#}");
        }
    }

    println!();
    println!("=================================================");
    println!("load_or_default<T>() – infallible");
    println!("file   : {file}");
    println!("format : TOML");
    println!("-------------------------------------------------");

    let value = load_or_default::<TestStruct>(file, None, format);
    println!("returned value:");
    println!("{value:#?}");
    println!("=================================================");
}
