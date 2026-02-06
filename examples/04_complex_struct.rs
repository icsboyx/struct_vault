//! Example 4: Complex struct with nested types.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use struct_vault::{vault_config, PersistentStructConfig, SaveType};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

#[vault_config]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct ComplexConfig {
    user_name: String,
    age: u32,
    scores: Vec<f64>,
    address: Option<Address>,
    metadata: HashMap<String, String>,
    tags: Vec<String>,
}

fn main() {
    let mut config = ComplexConfig {
        user_name: "Mario Rossi".to_string(),
        age: 30,
        scores: vec![95.5, 87.3, 92.1, 88.7],
        address: Some(Address {
            street: "Via Roma 123".to_string(),
            city: "Milano".to_string(),
            zip: "20100".to_string(),
        }),
        metadata: HashMap::from([
            ("role".to_string(), "admin".to_string()),
            ("department".to_string(), "IT".to_string()),
        ]),
        tags: vec!["developer".to_string(), "rust".to_string(), "senior".to_string()],
        ..Default::default()
    };

    config
        .vault_config_builder("complex_config")
        .set_file_type(SaveType::Json)
        .set_save_dir(".test_vault/complex");

    config.vault_save().expect("Failed to save complex config");

    let path = config.vault_config().unwrap().path();
    let content = fs::read_to_string(&path).expect("Failed to read file");
    println!("JSON content:\n{}", content);
}
