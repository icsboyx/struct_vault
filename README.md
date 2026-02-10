# struct_vault

Simple persistence of Rust structs to disk using Serde.

Struct Vault provides a small API for saving and loading structs in JSON, YAML, or TOML. It creates the target directory if needed and returns rich errors via `anyhow`.

## Features

- Save and load with `save`, `load`, and `load_or_default`
- Formats: JSON, YAML, TOML
- Default directory: `.config` (relative to the working directory)
- Optional convenience trait for type-based filenames

## Install

```bash
cargo add struct_vault
```

## Quick Start

```rust
use serde::{Deserialize, Serialize};
use struct_vault::{SaveFormat, load, save};

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

fn main() -> anyhow::Result<()> {
    let file = "app_config";
    let format = SaveFormat::Toml;

    let cfg = AppConfig {
        name: "demo".to_string(),
        value: 42,
    };

    save(&cfg, file, None, format)?;
    let loaded = load::<AppConfig>(file, None, format)?;

    println!("{loaded:#?}");
    Ok(())
}
```

## API Overview

```rust
use struct_vault::{SaveFormat, load, load_or_default, save};

// Save
save(&value, "file_name", None, SaveFormat::Toml)?;

// Load (errors on missing/invalid file)
let value = load::<MyType>("file_name", None, SaveFormat::Toml)?;

// Load or default (returns T::default() on any error)
let value = load_or_default::<MyType>("file_name", None, SaveFormat::Toml);
```

### Formats

`SaveFormat` controls the file extension and serializer:

- `SaveFormat::Json` -> `.json`
- `SaveFormat::Yaml` -> `.yaml`
- `SaveFormat::Toml` -> `.toml`

The default is TOML.

### Directories and Files

`dir: Option<&str>` controls where the file is stored:

- `None` uses `.config` (created if missing)
- `Some("path")` uses the provided directory (created if missing)

`file_name` is used as the base name; the chosen format determines the extension.

## StructVaultSimple Trait

If you want a minimal, type-based API, implement `StructVaultSimple`:

```rust
use serde::{Deserialize, Serialize};
use struct_vault::StructVaultSimple;

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    value: i32,
}

impl StructVaultSimple for AppConfig {}

fn main() -> anyhow::Result<()> {
    let mut cfg = AppConfig::default();

    cfg.load_or_default();
    cfg.name = "updated".into();
    cfg.save()?;
    cfg.load()?;

    Ok(())
}
```

`StructVaultSimple` uses:

- `vault_filename()` derived from the type name (last path segment)
- `.config` as the directory
- TOML as the format

## Examples

Examples live in `examples/`:

- `examples/01_simple_load.rs`
- `examples/02_simple_save.rs`
- `examples/03_simple_load_or_default copy.rs`
- `examples/04_simple_struct_vault_simple.rs`

Run one with:

```bash
cargo run --example 02_simple_save
```

## License

MIT. See [LICENSE](LICENSE).
