//! Example 6: Error handling - corrupted file.

use serde::{Deserialize, Serialize};
use std::fs;
use struct_vault::{vault_config, PersistentStructConfig, SaveType};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct SimpleConfig {
    name: String,
    count: i32,
    enabled: bool,
}

fn main() {
    let corrupt_dir = ".test_vault/corrupted";
    fs::create_dir_all(corrupt_dir).expect("Failed to create dir");
    fs::write(
        format!("{}/corrupted_config.json", corrupt_dir),
        "{ this is not valid json !!!",
    )
    .expect("Failed to write corrupted file");

    let mut config = SimpleConfig::default();
    config
        .vault_config_builder("corrupted_config")
        .set_file_type(SaveType::Json)
        .set_save_dir(corrupt_dir);

    match config.vault_load() {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected deserialization error: {}", e),
    }
}
