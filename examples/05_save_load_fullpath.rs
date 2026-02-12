use anyhow::Result;
use serde::{Deserialize, Serialize};
use struct_vault::StructVaultSimple;

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppState {
    profile: String,
    level: u32,
}

impl StructVaultSimple for AppState {}

fn main() -> Result<()> {
    let path = ".config/full_path_demo/custom_state.toml";

    let mut state = AppState {
        profile: "player-two".into(),
        level: 99,
    };

    state.save_to_path(path)?;
    println!("Saved to path: {path}");
    println!("{state:#?}");

    state = AppState::default();
    state.load_from_path(path)?;
    println!("Loaded from path: {path}");
    println!("{state:#?}");

    Ok(())
}
