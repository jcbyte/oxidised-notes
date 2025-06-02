use crate::storage;
use colored::Colorize;

pub(crate) struct Notes {
    notes: Vec<String>,
    filename: String,
    dirty: bool,
}

impl Notes {
    pub fn new(filename: String) -> Self {
        let notes = storage::load(&filename).unwrap(); // todo handle this safely
        Self {
            notes,
            filename,
            dirty: false,
        }
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

    pub fn delete(&mut self, idx: usize) {
        // todo show error when not in bounds
        self.notes.remove(idx);
        self.dirty = true;
    }
}

impl Drop for Notes {
    fn drop(&mut self) {
        if self.dirty {
            storage::save(&self.filename, &self.notes); // todo handle this safely
        }
    }
}
