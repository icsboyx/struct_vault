//! Example 14: Async usage with Tokio.

use serde::{Deserialize, Serialize};
use struct_vault::{async_feature, vault_config, PersistentStructConfig, SaveType};

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct AsyncConfig {
    name: String,
    count: i32,
    enabled: bool,
}

#[tokio::main]
async fn main() {
    let mut config = AsyncConfig::default();
    config
        .vault_config_builder("async_example")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/async");

    async_feature::vault_load_or_default_async(&mut config).await;
    println!("Loaded (or default): {:?}", config);

    config.count += 1;
    config.name = format!("Async run #{}", config.count);
    config.enabled = !config.enabled;

    async_feature::vault_save_async(&config)
        .await
        .expect("Failed to save async");

    let mut reloaded = AsyncConfig::default();
    reloaded
        .vault_config_builder("async_example")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/async");
    async_feature::vault_load_async(&mut reloaded)
        .await
        .expect("Failed to load async");

    println!("Reloaded: {:?}", reloaded);
}
