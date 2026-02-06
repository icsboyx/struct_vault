#[cfg(feature = "derive")]
pub use struct_vault_macros::vault_config;

pub static DEFAULT_SAVE_DIR: &str = ".config";

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy)]
pub enum SaveType {
    Json,
    #[default]
    Toml,
    Yaml,
}

impl SaveType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SaveType::Json => "json",
            SaveType::Toml => "toml",
            SaveType::Yaml => "yaml",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructSaveConfig {
    id: Uuid,
    file_name: String,
    save_dir: String,
    full_path: String,
    file_type: SaveType,
}
impl StructSaveConfig {
    pub fn set_file_name(&mut self, file_name: &str) -> &mut Self {
        self.file_name = file_name.into();
        self.update_full_path();
        self
    }
    pub fn set_save_dir(&mut self, save_dir: &str) -> &mut Self {
        self.save_dir = save_dir.into();
        self.update_full_path();
        self
    }
    pub fn set_file_type(&mut self, file_type: SaveType) -> &mut Self {
        self.file_type = file_type;
        self.update_full_path();
        self
    }

    pub fn config(&mut self, save_dir: Option<String>, file_name: Option<String>, file_type: Option<SaveType>) {
        self.file_name = file_name.unwrap_or_else(|| self.get_id().to_string());
        self.save_dir = save_dir.unwrap_or_else(|| DEFAULT_SAVE_DIR.to_string());
        self.file_type = file_type.unwrap_or(SaveType::default());
        self.update_full_path();
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_file_name(&self) -> &String {
        &self.file_name
    }

    pub fn get_save_dir(&self) -> &String {
        &self.save_dir
    }

    pub fn get_file_type(&self) -> SaveType {
        self.file_type
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.full_path.to_owned())
    }

    fn update_full_path(&mut self) {
        self.full_path = format!("{}/{}.{}", self.save_dir, self.file_name, self.file_type.as_str());
    }

    fn default(file_name: &str) -> Self {
        let id = Uuid::new_v4();
        let save_dir = DEFAULT_SAVE_DIR.to_string();
        let file_type = SaveType::default();
        let full_path = format!("{}/{}.{}", save_dir, file_name, file_type.as_str());
        StructSaveConfig {
            id,
            file_name: file_name.into(),
            save_dir,
            full_path,
            file_type: SaveType::default(),
        }
    }
}

pub trait PersistentStructConfig: Sized + Serialize + for<'de> Deserialize<'de> + 'static {
    fn vault_config(&self) -> Option<&StructSaveConfig>;
    fn vault_get_config(&mut self) -> &mut Option<StructSaveConfig>;

    // Set default config with given file name
    fn vault_config_default(&mut self, file_name: &str) -> &mut Self {
        *self.vault_get_config() = Some(StructSaveConfig::default(file_name));
        self
    }

    // Get mutable reference to the config, initializing it if necessary
    fn vault_config_builder(&mut self, file_name: &str) -> &mut StructSaveConfig {
        if self.vault_config().is_none() {
            *self.vault_get_config() = Some(StructSaveConfig::default(file_name));
        }
        self.vault_get_config().as_mut().unwrap()
    }

    // Load data from file specified in the config
    fn vault_load(&mut self) -> Result<()> {
        if let Some(config) = self.vault_config().cloned() {
            let path = config.path();
            let data = match fs::read_to_string(&path) {
                Ok(d) => d,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to read data from file {}: {}",
                        path.to_string_lossy(),
                        e
                    ));
                }
            };

            let deserialized = match Self::data_deserialize(&data, Some(config.get_file_type())) {
                Ok(d) => d,
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to deserialize data from file {}: {}",
                        path.to_string_lossy(),
                        e
                    ));
                }
            };
            *self = deserialized;
            *self.vault_get_config() = Some(config);
            return Ok(());
        } else {
            Err(anyhow::anyhow!("No vault config set"))
        }
    }

    // Load data from file if exists or error, otherwise use default values
    fn vault_load_or_default(&mut self)
    where
        Self: std::fmt::Debug,
    {
        if let Some(config) = self.vault_config().cloned() {
            let path = config.path();
            if path.exists() {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(deserialized) = Self::data_deserialize(&data, Some(config.get_file_type())) {
                        {
                            *self = deserialized;
                        }
                        *self.vault_get_config() = Some(config);
                    }
                }
            }
        } else {
            panic!("No vault config set");
        }
    }

    fn vault_save(&self) -> Result<()> {
        if let Some(config) = self.vault_config() {
            let path = config.path();
            let data = self.data_serialize(Some(config.get_file_type()))?;
            safe_write(&path, &data)
        } else {
            Err(anyhow::anyhow!("No vault config set"))
        }
    }

    fn data_deserialize(s: &str, file_type: Option<SaveType>) -> Result<Self> {
        let file_type = file_type.unwrap_or(SaveType::default());
        match file_type {
            SaveType::Json => Ok(serde_json::from_str(s)?),
            SaveType::Toml => Ok(toml::from_str(s)?),
            SaveType::Yaml => Ok(serde_yaml::from_str(s)?),
        }
    }

    fn data_serialize(&self, file_type: Option<SaveType>) -> Result<String> {
        let file_type = file_type.unwrap_or(SaveType::default());
        match file_type {
            SaveType::Json => Ok(serde_json::to_string_pretty(self)?),
            SaveType::Toml => Ok(toml::to_string_pretty(self)?),
            SaveType::Yaml => Ok(serde_yaml::to_string(self)?),
        }
    }
}

fn safe_write(path: &Path, data: &str) -> Result<()> {
    let dir = path.parent().ok_or_else(|| anyhow::anyhow!("No parent dir"))?;
    fs::create_dir_all(dir)?;
    let mut tmp = path.to_path_buf();
    tmp.set_extension(format!(
        "{}.tmp",
        path.extension().and_then(OsStr::to_str).unwrap_or("tmp")
    ));
    {
        let mut f = File::create(&tmp)?;
        f.write_all(data.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}
