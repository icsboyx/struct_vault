//! Async helpers for Struct Vault.
//!
//! These helpers mirror the sync API using Tokio for async IO.

use crate::PersistentStructConfig;
use anyhow::Result;
use std::{ffi::OsStr, path::Path};
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn vault_load_async<T: PersistentStructConfig>(value: &mut T) -> Result<()> {
    if let Some(config) = value.vault_config().cloned() {
        let path = config.path();
        let data = match fs::read_to_string(&path).await {
            Ok(d) => d,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to read data from file {}: {}",
                    path.to_string_lossy(),
                    e
                ));
            }
        };

        let deserialized = match T::data_deserialize(&data, Some(config.get_file_type())) {
            Ok(d) => d,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to deserialize data from file {}: {}",
                    path.to_string_lossy(),
                    e
                ));
            }
        };
        *value = deserialized;
        *value.vault_get_config() = Some(config);
        Ok(())
    } else {
        Err(anyhow::anyhow!("No vault config set"))
    }
}

pub async fn vault_load_or_default_async<T>(value: &mut T)
where
    T: PersistentStructConfig + std::fmt::Debug,
{
    if let Some(config) = value.vault_config().cloned() {
        let path = config.path();
        if fs::metadata(&path).await.is_ok() {
            if let Ok(data) = fs::read_to_string(&path).await {
                if let Ok(deserialized) = T::data_deserialize(&data, Some(config.get_file_type())) {
                    *value = deserialized;
                    *value.vault_get_config() = Some(config);
                }
            }
        }
    } else {
        panic!("No vault config set");
    }
}

pub async fn vault_save_async<T: PersistentStructConfig>(value: &T) -> Result<()> {
    if let Some(config) = value.vault_config() {
        let path = config.path();
        let data = value.data_serialize(Some(config.get_file_type()))?;
        safe_write_async(&path, &data).await
    } else {
        Err(anyhow::anyhow!("No vault config set"))
    }
}

async fn safe_write_async(path: &Path, data: &str) -> Result<()> {
    let dir = path.parent().ok_or_else(|| anyhow::anyhow!("No parent dir"))?;
    fs::create_dir_all(dir).await?;

    let mut tmp = path.to_path_buf();
    tmp.set_extension(format!(
        "{}.tmp",
        path.extension().and_then(OsStr::to_str).unwrap_or("tmp")
    ));

    {
        let mut f = fs::File::create(&tmp).await?;
        f.write_all(data.as_bytes()).await?;
        f.sync_all().await?;
    }
    fs::rename(&tmp, path).await?;
    Ok(())
}
