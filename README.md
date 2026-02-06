# Struct Vault

Struct Vault is a small Rust library to persist structs to disk with minimal setup. It supports TOML, JSON, and YAML, and provides a simple API to load, save, or load defaults when files are missing.

## What It’s For

- Application settings that should survive restarts
- CLI tools that need a simple config file
- Small utilities that want a drop-in persistence layer

## Key Capabilities

- Save and load structs in TOML, JSON, or YAML
- Default save location is `.config/`
- Custom file names and custom directories
- Safe writes (temporary file + rename)
- Optional derive macro for quick integration
 - Works with Send + Sync types; thread safety depends on your fields

## How It Works

Struct Vault injects a hidden config field into your struct (via `#[vault_config]`) and uses it to build a file path. When you call load/save, it serializes or deserializes your struct using the selected format, then writes or reads the file. Writes are done safely by writing to a temporary file and renaming it.

## Send + Sync

Struct Vault does not block multithread usage by itself. The injected config field is `Send + Sync`. Your struct is `Send + Sync` only if all its fields are `Send + Sync`. 

## Workspace Layout

- `struct_vault/` is the library crate
- `struct_vault_macros/` provides the derive macro
- `examples/` contains runnable usage examples

## Running Examples

Each example is a standalone binary. Run any of them with:

- `cargo run -p struct_vault_examples --bin 00_injected_field`
- `cargo run -p struct_vault_examples --bin 01_default_toml`
- `cargo run -p struct_vault_examples --bin 02_json_custom_dir`
- `cargo run -p struct_vault_examples --bin 03_yaml_format`
- `cargo run -p struct_vault_examples --bin 04_complex_struct`
- `cargo run -p struct_vault_examples --bin 05_error_missing_file`
- `cargo run -p struct_vault_examples --bin 06_error_corrupted_file`
- `cargo run -p struct_vault_examples --bin 07_error_no_config`
- `cargo run -p struct_vault_examples --bin 08_runtime_modification`
- `cargo run -p struct_vault_examples --bin 09_load_preserves_config`
- `cargo run -p struct_vault_examples --bin 10_format_comparison`
- `cargo run -p struct_vault_examples --bin 11_incremental_updates`
- `cargo run -p struct_vault_examples --bin 12_timestamp`
- `cargo run -p struct_vault_examples --bin 13_multithread`
- `cargo run -p struct_vault_examples --bin 14_async`
- `cargo run -p struct_vault_examples --bin 15_async_concurrency`
- `cargo run -p struct_vault_examples --bin 16_send_sync`

## Files Created by Examples

The examples creates/write files to:

- `.config/`
- `.test_vault/`

To clean up:

- `rm -rf .config .test_vault`

## Notes

- The derive macro is enabled by default in the library features.
- Async helpers are available behind the `async` feature.

## License

See [LICENSE](LICENSE).
