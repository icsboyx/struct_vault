//! Example 9: Load preserves the vault configuration.

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
    let mut original = SimpleConfig {
        name: "Original".to_string(),
        count: 500,
        enabled: true,
        ..Default::default()
    };
    original
        .vault_config_builder("preserve_test")
        .set_save_dir(".test_vault/preserve");
    original.vault_save().expect("Failed to save");

    let original_id = original.vault_config().unwrap().get_id();
    println!("Original UUID: {}", original_id);

    let mut loaded = SimpleConfig::default();
    loaded
        .vault_config_builder("preserve_test")
        .set_save_dir(".test_vault/preserve");

    let pre_load_id = loaded.vault_config().unwrap().get_id();
    println!("Pre-load UUID: {}", pre_load_id);

    loaded.vault_load().expect("Failed to load");

    let post_load_id = loaded.vault_config().unwrap().get_id();
    println!("Post-load UUID: {}", post_load_id);
    println!("Loaded values: {:?}", loaded);
}
