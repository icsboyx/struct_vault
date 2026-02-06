//! Example 11: Incremental updates (simulated real usage).

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
    let config_name = "counter_app";
    let config_dir = ".test_vault/incremental";

    let mut config = SimpleConfig::default();
    config.vault_config_builder(config_name).set_save_dir(config_dir);
    config.vault_load_or_default();

    println!("Before update - count: {}", config.count);
    config.count += 1;
    config.name = format!("Run #{}", config.count);
    config.vault_save().expect("Failed to save");

    let mut next_run = SimpleConfig::default();
    next_run
        .vault_config_builder(config_name)
        .set_save_dir(config_dir);
    next_run.vault_load_or_default();
    println!("Next run - count: {}", next_run.count);
    println!("Loaded name: {}", next_run.name);
}
