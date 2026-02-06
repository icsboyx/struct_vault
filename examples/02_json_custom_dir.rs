//! Example 2: JSON with a custom directory.

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
        .vault_config_builder("json_config")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/json_dir");

    println!("Path: {:?}", config.vault_config().unwrap().path());

    config.name = "JSON Config".to_string();
    config.count = 100;
    config.enabled = false;
    config.vault_save().expect("Failed to save JSON");

    let mut reloaded = SimpleConfig::default();
    reloaded
        .vault_config_builder("json_config")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/json_dir");
    reloaded.vault_load().expect("Failed to load JSON");
    println!("Reloaded: {:?}", reloaded);
}
