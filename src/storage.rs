use serde_json;
use std::{fs, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub(crate) fn load(filename: &String) -> Result<Vec<String>, StorageError> {
    if !fs::exists(filename)? {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(filename)?;
    let notes: Vec<String> = serde_json::from_str(&data)?;

    return Ok(notes);
}

pub(crate) fn save(filename: &String, notes: &Vec<String>) -> Result<(), StorageError> {
    let json = serde_json::to_string(notes)?;
    fs::write(filename, json.as_bytes())?;

    return Ok(());
}
