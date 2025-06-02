use crate::storage;

const FILENAME: &str = "notes.json";

pub(crate) struct Notes {
    notes: Vec<String>,
    dirty: bool,
}

impl Notes {
    // todo should filename be passed into constructor?
    pub fn new() -> Self {
        let notes = storage::load(FILENAME);
        Self {
            notes,
            dirty: false,
        }
    }

    // todo cannot add more than 256
    pub fn add(&mut self, content: &str) {
        self.notes.push(content.to_string());
        self.dirty = true;
    }

    pub fn list() {
        todo!();
    }

    pub fn edit() {
        todo!();
    }

    pub fn delete(&mut self, idx: usize) {
        self.notes.remove(idx);
        self.dirty = true;
    }
}

impl Drop for Notes {
    fn drop(&mut self) {
        if self.dirty {
            storage::save(FILENAME, &self.notes);
        }
    }
}
