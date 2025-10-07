#[cfg(test)]
mod tests;

pub(crate) static CONFIG_DB: LazyLock<ConfigDB> = LazyLock::new(ConfigDB::default);
use anyhow::Result;
use std::{
    any::TypeId,
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
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
pub struct StructConfig {
    type_id: TypeId,
    type_name: String,
    file_name: String,
    dir: PathBuf,
    path: PathBuf,
    file_type: SaveType,
}

impl StructConfig {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
    pub fn dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    pub fn file_type(&self) -> &SaveType {
        &self.file_type
    }

    pub fn type_id(&self) -> &TypeId {
        &self.type_id
    }
}

#[derive(Debug, Default)]
pub struct ConfigDB {
    configs: RwLock<HashMap<TypeId, StructConfig>>,
}

impl ConfigDB {
    pub fn add_config<T>(&self, config: StructConfig)
    where
        T: 'static,
    {
        let type_id = std::any::TypeId::of::<T>();
        self.configs.write().unwrap().insert(type_id, config);
    }

    pub fn get_config<T>(&self) -> Option<StructConfig>
    where
        T: 'static,
    {
        let type_id = std::any::TypeId::of::<T>();
        self.configs.read().unwrap().get(&type_id).cloned()
    }

    pub fn contains<T>(&self) -> bool
    where
        T: 'static,
    {
        let type_id = std::any::TypeId::of::<T>();
        self.configs.read().unwrap().contains_key(&type_id)
    }
}

pub trait PersistentStructConfig: Sized + Serialize + for<'de> Deserialize<'de> + 'static {
    fn config(&self, dir: Option<&str>, file_name: Option<&str>, file_type: Option<SaveType>) {
        let type_id = TypeId::of::<Self>();
        let type_name = std::any::type_name::<Self>().split("::").last().unwrap().to_string();

        let file_name = file_name.map(|s| s.to_string()).unwrap_or(type_name.to_string());
        let dir = PathBuf::from(dir.map(|d| d.to_string()).unwrap_or(".config".to_string()));
        let file_type = file_type.unwrap_or(SaveType::default());
        let path = PathBuf::from(&dir).join(format!("{}.{}", &file_name, file_type.as_str()));

        let config = StructConfig {
            type_id,
            type_name,
            file_name,
            dir,
            path,
            file_type,
        };

        CONFIG_DB.add_config::<Self>(config);
    }

    fn config_default(&self) {
        self.config(None, None, None);
    }

    fn configured(&self) -> bool {
        CONFIG_DB.contains::<Self>()
    }

    fn load_or_default(&mut self) -> &mut Self
    where
        Self: Default,
    {
        *self = self.load().unwrap_or_default();
        self
    }

    fn try_load_into(&mut self) -> Result<&mut Self> {
        *self = self.load()?;
        Ok(self)
    }

    fn load(&self) -> Result<Self> {
        let Some(config) = CONFIG_DB.get_config::<Self>() else {
            return Err(anyhow::anyhow!("Struct Vault not configured"));
        };
        let content = std::fs::read_to_string(config.path())?;
        let deserialized = Self::data_deserialize(&content, Some(config.file_type.clone()))
            .map_err(|e| anyhow::anyhow!("Deserialization error: {}", e))?;
        Ok(deserialized)
    }
    fn save(&self) -> Result<()> {
        let Some(config) = CONFIG_DB.get_config::<Self>() else {
            return Err(anyhow::anyhow!("Struct Vault not configured"));
        };
        let serialized = self.data_serialize(Some(config.file_type.clone()))?;
        safe_write(config.path(), &serialized)?;
        Ok(())
    }

    fn data_serialize(&self, file_type: Option<SaveType>) -> Result<String> {
        let file_type = file_type.unwrap_or(SaveType::default());
        match file_type {
            SaveType::Json => Ok(serde_json::to_string_pretty(self)?),
            SaveType::Toml => Ok(toml::to_string_pretty(self)?),
            SaveType::Yaml => Ok(serde_yaml::to_string(self)?),
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
