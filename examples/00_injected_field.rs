//! Example 00: Compare serialization with and without the injected vault field.

use serde::{Deserialize, Serialize};
use struct_vault::{PersistentStructConfig, vault_config};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct WithVault {
    name: String,
    count: i32,
    enabled: bool,
}

fn main() {
    let mut with_vault = WithVault::default();
    with_vault.vault_config_default("injected_example");

    println!(
        "Struct config values are skipped by serde:\n{}",
        serde_json::to_string_pretty(&with_vault).unwrap()
    );

    println!(
        "With #[vault_config] (injected field is skipped by serde):\n{:#?}",
        with_vault
    );
}
