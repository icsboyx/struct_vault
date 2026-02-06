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

## Workspace Layout

- `struct_vault/` is the library crate
- `struct_vault_macros/` provides the derive macro
- `examples/` contains runnable usage examples

## Running Examples

Each example is a standalone binary. Run any of them with:

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

## Files Created by Examples

The examples write files to:

- `.config/`
- `.test_vault/`

To clean up:

- `rm -rf .config .test_vault`

## Notes

- The derive macro is enabled by default in the library features.
- All examples are written in English and focused on practical usage.
