//! Example 10: Compare formats with the same data.

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
    let test_data = SimpleConfig {
        name: "Format Example".to_string(),
        count: 12345,
        enabled: true,
        ..Default::default()
    };

    let formats = [
        ("TOML", SaveType::Toml, "toml"),
        ("JSON", SaveType::Json, "json"),
        ("YAML", SaveType::Yaml, "yaml"),
    ];

    for (name, save_type, ext) in formats {
        let mut config = test_data.clone();
        config
            .vault_config_builder(&format!("format_example_{}", ext))
            .set_file_type(save_type)
            .set_save_dir(".test_vault/formats");
        config.vault_save().expect("Failed to save");

        let path = config.vault_config().unwrap().path();
        let content = fs::read_to_string(&path).expect("Failed to read");

        println!("\n{}", name);
        println!("{}", content);
    }
}
