//! Example 12: Practical example with a timestamp.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
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
        .vault_config_builder("app_settings")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/practical");

    config.vault_load_or_default();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    config.name = format!("Last updated (unix): {}", now);
    config.count += 1;
    config.enabled = !config.enabled;

    config.vault_save().expect("Failed to save");

    println!("Saved config:");
    println!("  Name: {}", config.name);
    println!("  Count: {} (execution count)", config.count);
    println!("  Enabled: {} (toggled)", config.enabled);
}
