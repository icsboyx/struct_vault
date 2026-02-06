//! Example 16: Compile-time proof of Send + Sync bounds.

use serde::{Deserialize, Serialize};
use struct_vault::{vault_config, PersistentStructConfig};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct ThreadSafeConfig {
    name: String,
    count: i32,
    enabled: bool,
}

fn assert_send_sync<T: Send + Sync>() {}

fn main() {
    // If this compiles, the type is Send + Sync.
    assert_send_sync::<ThreadSafeConfig>();

    let mut cfg = ThreadSafeConfig::default();
    cfg.vault_config_default("send_sync_example");
    cfg.vault_load_or_default();
    cfg.count += 1;
    cfg.vault_save().expect("save");

    println!("Send + Sync check passed: {:?}", cfg);
}
