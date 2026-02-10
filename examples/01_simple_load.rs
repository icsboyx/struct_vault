use std::process::exit;

use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, load};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TestStruct {
    pub name: String,
    pub value: i32,
}

fn main() {
    let file = "test_struct";

    println!("=================================================");
    println!("Loading data");
    println!("file   : {file}");
    println!("format : TOML");
    println!("-------------------------------------------------");

    let test_struct = load::<TestStruct>(file, None, SaveFormat::Toml).unwrap_or_else(|e| {
        println!("ERROR while loading:");
        println!("{e:#}");
        exit(2);
    });

    println!("OK");
    println!("{test_struct:#?}");
    println!("=================================================");
}
