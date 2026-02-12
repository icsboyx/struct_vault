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
    let dir = ".config/slot_a";

    let mut state = AppState {
        profile: "player-one".into(),
        level: 10,
    };

    state.save_in(dir)?;
    println!("Saved in dir: {dir}");
    println!("{state:#?}");

    state = AppState::default();
    state.load_from(dir)?;
    println!("Loaded from dir: {dir}");
    println!("{state:#?}");

    Ok(())
}
