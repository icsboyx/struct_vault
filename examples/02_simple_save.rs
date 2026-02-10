use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, load, save};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TestStruct {
    pub name: String,
    pub value: i32,
}

fn main() {
    let file = "test_struct";
    let format = SaveFormat::default();

    let struct_to_save = TestStruct {
        name: "Test".to_string(),
        value: 42,
    };

    println!("=================================================");
    println!("Saving data");
    println!("file   : {file}");
    println!("format : {:?}", format);
    println!("-------------------------------------------------");
    println!("{struct_to_save:#?}");

    save(&struct_to_save, file, None, format).unwrap_or_else(|e| {
        eprintln!("ERROR while saving:\n{e:#}");
    });

    println!();
    println!("=================================================");
    println!("Loading saved data");
    println!("file   : {file}");
    println!("format : {:?}", format);
    println!("-------------------------------------------------");

    match load::<TestStruct>(file, None, format) {
        Ok(value) => {
            println!("OK");
            println!("{value:#?}");
        }
        Err(e) => {
            eprintln!("ERROR while loading:\n{e:#}");
        }
    }

    println!("=================================================");
}
