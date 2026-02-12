# struct_vault

Simple persistence of Rust structs to disk using Serde.

`struct_vault` provides small helpers to save/load structs in JSON, YAML, or TOML.

## Install

`struct_vault` is currently not published on crates.io.

```toml
[dependencies]
struct_vault = { git = "https://github.com/icsboyx/struct_vault" }
```

## Basic Example

```rust
use anyhow::Result;
use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, load, save};

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

fn main() -> Result<()> {
    let cfg = AppConfig {
        name: "demo".into(),
        value: 42,
    };

    save(&cfg, "app_config", None, SaveFormat::Toml)?;
    let loaded = load::<AppConfig>("app_config", None, SaveFormat::Toml)?;

    println!("{loaded:#?}");
    Ok(())
}
```

## Notes

- Default directory is `.config`.
- `StructVaultSimple` is available for quick type-based save/load.
- You can also use explicit path APIs: `save_to_path` and `load_from_path`.

## Examples

- `examples/01_simple_load.rs`
- `examples/02_simple_save.rs`
- `examples/03_simple_load_or_default.rs`
- `examples/04_simple_struct_vault_simple.rs`
- `examples/05_custom_dir.rs`
- `examples/06_save_in_load_from.rs`
- `examples/05_save_load_fullpath.rs`

Run one with:

```bash
cargo run --example 05_save_load_fullpath
```

## License

MIT. See [LICENSE](LICENSE).
