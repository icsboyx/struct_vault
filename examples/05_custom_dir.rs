use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, StructVaultSimple};

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppState {
    profile: String,
    level: u32,
}

impl StructVaultSimple for AppState {}

fn main() -> Result<()> {
    let custom_dir = ".config/custom_example";
    AppState::set_custom_dir(custom_dir);

    let mut state = AppState {
        profile: "player-one".into(),
        level: 7,
    };

    println!("=================================================");
    println!("Custom global directory demo");
    println!("dir    : {custom_dir}");
    println!("file   : {}", AppState::vault_filename());
    println!("format : {:?}", SaveFormat::Toml);
    println!("-------------------------------------------------");

    state.save()?;
    println!("Saved:");
    println!("{state:#?}");

    state = AppState::default();
    state.load()?;

    let mut path = PathBuf::from(custom_dir);
    path.push(AppState::vault_filename());
    path.set_extension(SaveFormat::Toml.as_ext());

    println!();
    println!("Loaded from: {}", path.display());
    println!("{state:#?}");
    println!("=================================================");

    Ok(())
}
