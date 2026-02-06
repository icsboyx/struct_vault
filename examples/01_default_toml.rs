//! Example 1: Default TOML configuration using #[vault_config].

use serde::{Deserialize, Serialize};
use struct_vault::{vault_config, PersistentStructConfig};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct SimpleConfig {
    name: String,
    count: i32,
    enabled: bool,
}

fn main() {
    let mut config = SimpleConfig::default();
    config.vault_config_default("simple_default");

    let vault = config.vault_config().unwrap();
    println!("File path: {:?}", vault.path());
    println!("File name: {}", vault.get_file_name());
    println!("Save dir: {}", vault.get_save_dir());
    println!("UUID: {}", vault.get_id());

    config.vault_load_or_default();
    println!("Loaded (or default): {:?}", config);

    config.name = "TOML Example".to_string();
    config.count = 42;
    config.enabled = true;
    config.vault_save().expect("Failed to save");

    let mut reloaded = SimpleConfig::default();
    reloaded.vault_config_default("simple_default");
    reloaded.vault_load().expect("Failed to reload");
    println!("Reloaded: {:?}", reloaded);
}
