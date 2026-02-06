//! Example 5: Error handling - file not found.

use serde::{Deserialize, Serialize};
use struct_vault::{vault_config, PersistentStructConfig, SaveType};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct SimpleConfig {
    name: String,
    count: i32,
    enabled: bool,
}

fn main() {
    let mut config = SimpleConfig::default();
    config
        .vault_config_builder("missing_file")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/missing");

    match config.vault_load() {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {}", e),
    }
}
