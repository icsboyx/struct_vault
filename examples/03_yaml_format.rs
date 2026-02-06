//! Example 3: YAML format.

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
    let mut config = SimpleConfig::default();
    config
        .vault_config_builder("yaml_config")
        .set_file_type(SaveType::Yaml)
        .set_save_dir(".test_vault/yaml_dir");

    config.name = "YAML Config".to_string();
    config.count = 999;
    config.enabled = true;
    config.vault_save().expect("Failed to save YAML");

    let path = config.vault_config().unwrap().path();
    let content = fs::read_to_string(&path).expect("Failed to read YAML file");
    println!("YAML content:\n{}", content);
}
