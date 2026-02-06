//! Example 8: Runtime configuration modification.

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
    config.name = "Runtime Example".to_string();
    config.count = 1;

    config
        .vault_config_builder("runtime_config")
        .set_save_dir(".test_vault/runtime");
    config.vault_save().expect("Failed to save TOML");
    println!("Saved as TOML");

    config
        .vault_config_builder("runtime_config")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/runtime");
    config.vault_save().expect("Failed to save JSON");
    println!("Saved as JSON");

    config
        .vault_config_builder("runtime_config_renamed")
        .set_file_type(SaveType::Yaml)
        .set_save_dir(".test_vault/runtime/subdir");
    config.vault_save().expect("Failed to save YAML");
    println!("Saved as YAML in subdir with new name");
}
