use std::{
    ffi::OsStr,
    fmt::Display,
    fs::{create_dir_all, read, write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::Deserialize;

pub static DEFAULT_SAVE_DIR: &str = ".config";

#[derive(Debug, Clone, Copy, Default)]
pub enum SaveFormat {
    Json,
    Yaml,
    #[default]
    Toml,
}

impl SaveFormat {
    pub fn as_ext(&self) -> &'static str {
        match self {
            SaveFormat::Json => "json",
            SaveFormat::Yaml => "yaml",
            SaveFormat::Toml => "toml",
        }
    }
}

impl AsRef<OsStr> for SaveFormat {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_ext())
    }
}

impl Display for SaveFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ext())
    }
}

fn build_path(file_name: &str, dir: Option<&str>, format: SaveFormat) -> Result<PathBuf> {
    let dir = dir.unwrap_or(DEFAULT_SAVE_DIR);

    create_dir_all(dir).with_context(|| format!("Failed to create directory '{}'", dir))?;

    let mut path = PathBuf::from(dir);
    path.push(file_name);
    path.set_extension(format.as_ext());

    Ok(path)
}

pub fn load<T>(file_name: &str, dir: Option<&str>, format: SaveFormat) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let path = build_path(file_name, dir, format).context("Failed to build path for loading")?;

    let buffer = read(&path).context(format!("Failed to read file '{}'", path.display()))?;

    let data = to_struct(format, buffer)?;

    Ok(data)
}

pub fn load_or_default<T>(file_name: &str, dir: Option<&str>, format: SaveFormat) -> T
where
    T: Default + for<'de> Deserialize<'de>,
{
    match load(file_name, dir, format) {
        Ok(value) => value,
        Err(_) => T::default(),
    }
}

pub fn save<T>(data: &T, file_name: &str, dir: Option<&str>, format: SaveFormat) -> Result<()>
where
    T: serde::Serialize,
{
    let path = build_path(file_name, dir, format).context("Failed to build path for saving")?;

    let buffer = to_string(data, format)?;

    write(&path, buffer).with_context(|| format!("Failed to write file '{}'", path.display()))?;

    Ok(())
}

fn to_string<T>(data: &T, format: SaveFormat) -> Result<String, anyhow::Error>
where
    T: serde::Serialize,
{
    let buffer = match format {
        SaveFormat::Json => serde_json::to_string_pretty(data).context("Failed to serialize data to JSON")?,
        SaveFormat::Yaml => serde_yaml::to_string(data).context("Failed to serialize data to YAML")?,
        SaveFormat::Toml => toml::to_string_pretty(data).context("Failed to serialize data to TOML")?,
    };
    Ok(buffer)
}

fn to_struct<T>(format: SaveFormat, buffer: Vec<u8>) -> Result<T, anyhow::Error>
where
    T: for<'de> Deserialize<'de>,
{
    let data = match format {
        SaveFormat::Json => serde_json::from_slice(&buffer).context("Failed to deserialize JSON")?,
        SaveFormat::Yaml => serde_yaml::from_slice(&buffer).context("Failed to deserialize YAML")?,
        SaveFormat::Toml => toml::from_slice(&buffer).context("Failed to deserialize TOML")?,
    };
    Ok(data)
}

pub trait StructVaultSimple: Sized {
    fn vault_filename() -> &'static str {
        let fqdn = std::any::type_name::<Self>();
        fqdn.rsplit("::").next().unwrap_or(fqdn)
    }

    fn load(&mut self) -> Result<()>
    where
        Self: for<'de> Deserialize<'de>,
    {
        let filetype = SaveFormat::default();
        *self = load(Self::vault_filename(), None, filetype)?;
        Ok(())
    }

    fn load_or_default(&mut self)
    where
        Self: Default + for<'de> Deserialize<'de>,
    {
        *self = load_or_default(Self::vault_filename(), None, SaveFormat::Toml);
    }

    fn save(&self) -> Result<()>
    where
        Self: serde::Serialize,
    {
        save(self, Self::vault_filename(), None, SaveFormat::Toml)?;
        Ok(())
    }
}
