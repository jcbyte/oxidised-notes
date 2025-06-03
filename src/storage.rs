use serde_json;
use std::{fs, io, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub(crate) fn load(path: &PathBuf) -> Result<Vec<String>, StorageError> {
    if !fs::exists(path)? {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(path)?;
    let notes: Vec<String> = serde_json::from_str(&data)?;

    return Ok(notes);
}

pub(crate) fn save(path: &PathBuf, notes: &Vec<String>) -> Result<(), StorageError> {
    let json = serde_json::to_string(notes)?;
    fs::write(path, json.as_bytes())?;

    return Ok(());
}
