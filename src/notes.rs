use crate::storage;
use colored::Colorize;

// todo should be relative to the exe
const FILENAME: &str = "notes.json";

pub(crate) struct Notes {
    notes: Vec<String>,
    dirty: bool,
}

impl Notes {
    // todo should filename be passed into constructor?
    pub fn new() -> Self {
        let notes = storage::load(FILENAME).unwrap(); // todo handle this safely
        Self {
            notes,
            dirty: false,
        }
    }

    pub fn add(&mut self, content: &str) {
        self.notes.push(content.to_string());
        self.dirty = true;
    }

    pub fn list(&self) {
        for (idx, note) in self.notes.iter().enumerate() {
            let index_part = format!("{:>2}:", idx + 1);
            println!("{} {}", index_part.bright_black().bold(), note)
        }
    }

    pub fn delete(&mut self, idx: usize) {
        self.notes.remove(idx);
        self.dirty = true;
    }
}

impl Drop for Notes {
    fn drop(&mut self) {
        if self.dirty {
            storage::save(FILENAME, &self.notes); // todo handle this safely
        }
    }
}
