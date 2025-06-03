use std::error::Error;

use crate::storage;
use colored::Colorize;

pub(crate) struct Notes {
    notes: Vec<String>,
    filename: String,
    dirty: bool,
}

impl Notes {
    pub fn new(filename: String) -> Result<Self, Box<dyn Error>> {
        let notes = storage::load(&filename)?;

        return Ok(Self {
            notes,
            filename,
            dirty: false,
        });
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        if self.dirty {
            storage::save(&self.filename, &self.notes)?;
        }
        self.dirty = false;

        return Ok(());
    }

    pub fn add(&mut self, content: String) {
        self.notes.push(content);
        self.dirty = true;
    }

    pub fn list(&self) {
        if self.notes.is_empty() {
            println!("{}", "No Notes Yet!".bright_black().bold());
            return;
        }

        for (idx, note) in self.notes.iter().enumerate() {
            let index_part = format!("{:>2}:", idx + 1);
            println!("{} {}", index_part.bright_black().bold(), note)
        }
    }

    pub fn delete(&mut self, idx: usize) -> Result<(), String> {
        if idx <= 0 || idx >= self.notes.len() {
            return Err(format!(
                "Note {} does not exist!",
                idx.to_string().bright_red().bold()
            ));
        }

        self.notes.remove(idx);
        self.dirty = true;

        return Ok(());
    }
}

#[cfg(debug_assertions)]
impl Drop for Notes {
    fn drop(&mut self) {
        if self.dirty {
            panic!("Notes was dropped with unsaved changes!");
        }
    }
}
