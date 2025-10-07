use serde::{Deserialize, Serialize};
use struct_vault::{
    PersistentStructConfig,
    SaveType::{Json, Toml, Yaml},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TestStruct {
    pub field1: String,
    pub field2: i32,
}

impl PersistentStructConfig for TestStruct {}

fn main() {
    // Using default configuration: directory ".config", filename "TestStruct", file type Toml
    let mut test_struct_default = TestStruct::default();
    test_struct_default.config_default();

    test_struct_default.load_or_default();
    println!("Loaded test_struct_default: {:?}", test_struct_default);

    test_struct_default.field1 = format!("Updated at {:?}", chrono::offset::Local::now());
    test_struct_default.field2 += 1;
    test_struct_default.save().unwrap();

    println!("Saved test_struct_default: {:?}", test_struct_default);

    // Using custom configuration toml file in ".config" directory
    let mut test_struct_toml = TestStruct::default();
    test_struct_toml.config(Some(".config"), Some("test_config_toml"), Some(Toml));

    test_struct_toml.load_or_default();
    println!("Loaded test_struct_toml: {:?}", test_struct_toml);

    test_struct_toml.field1 = format!("Updated at {:?}", chrono::offset::Local::now());
    test_struct_toml.field2 += 10;
    test_struct_toml.save().unwrap();
    println!("Saved test_struct_toml: {:?}", test_struct_toml);

    // Using custom configuration json file in ".config" directory
    let mut test_struct_json = TestStruct::default();
    test_struct_json.config(Some(".config"), Some("test_config_json"), Some(Json));
    test_struct_json.load_or_default();
    println!("Loaded test_struct_json: {:?}", test_struct_json);
    test_struct_json.field1 = format!("Updated at {:?}", chrono::offset::Local::now());
    test_struct_json.field2 += 100;
    test_struct_json.save().unwrap();
    println!("Saved test_struct_json: {:?}", test_struct_json);

    // Using custom configuration yaml file in ".config" directory
    let mut test_struct_yaml = TestStruct::default();
    test_struct_yaml.config(Some(".config"), Some("test_config_yaml"), Some(Yaml));
    test_struct_yaml.load_or_default();
    println!("Loaded test_struct_yaml: {:?}", test_struct_yaml);
    test_struct_yaml.field1 = format!("Updated at {:?}", chrono::offset::Local::now());
    test_struct_yaml.field2 += 1000;
    test_struct_yaml.save().unwrap();
    println!("Saved test_struct_yaml: {:?}", test_struct_yaml);
}
