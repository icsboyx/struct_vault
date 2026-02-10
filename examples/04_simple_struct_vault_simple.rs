use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, StructVaultSimple};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TestStruct {
    pub name: String,
    pub value: i32,
}

impl StructVaultSimple for TestStruct {}

fn main() -> Result<()> {
    let mut test_struct = TestStruct::default();
    let format = SaveFormat::default();

    println!("=================================================");
    println!("Setup: remove existing file (if any)");
    println!("-------------------------------------------------");

    let mut default_path = PathBuf::from(".config");
    default_path.push(TestStruct::vault_filename());
    default_path.set_extension(format.as_ext());

    println!("path   : {}", default_path.display());
    println!("format : {:?}", format);
    println!("-------------------------------------------------");

    match std::fs::remove_file(&default_path) {
        Ok(_) => println!("Removed existing file"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => println!("No existing file (ok)"),
        Err(e) => println!("Failed to remove file:\n{:#}", e),
    }

    println!();
    println!("=================================================");
    println!("1) load() – expected to fail");
    println!("-------------------------------------------------");

    match test_struct.load() {
        Ok(_) => {
            println!("OK (unexpected in this demo)");
            println!("{test_struct:#?}");
        }
        Err(e) => {
            println!("ERROR (expected)");
            println!("{e:#}");
        }
    }

    println!();
    println!("=================================================");
    println!("2) load_or_default() – infallible");
    println!("-------------------------------------------------");

    test_struct.load_or_default();
    println!("Returned value:");
    println!("{test_struct:#?}");

    println!();
    println!("=================================================");
    println!("3) modify + save()");
    println!("-------------------------------------------------");

    test_struct.name = "some data".into();
    test_struct.value = 42;
    println!("Saving:");
    println!("{test_struct:#?}");
    test_struct.save()?;
    println!("Saved.");

    println!();
    println!("=================================================");
    println!("4) reset + load()");
    println!("-------------------------------------------------");

    test_struct = TestStruct::default();
    println!("After reset:");
    println!("{test_struct:#?}");

    test_struct.load()?;
    println!("After load:");
    println!("{test_struct:#?}");

    println!("=================================================");

    Ok(())
}
