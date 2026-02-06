//! Example 7: Error handling - missing vault configuration.

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
    let config = SimpleConfig::default();
    match config.vault_save() {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error (no config): {}", e),
    }
}
