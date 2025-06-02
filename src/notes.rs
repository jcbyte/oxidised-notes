use crate::storage;

const FILENAME: &str = "notes.json";

struct Notes {
    notes: Vec<String>,
    dirty: bool,
}

impl Notes {
    // todo should filename be passed into constructor?
    fn new() -> Self {
        let notes = storage::load(FILENAME);
        Self {
            notes,
            dirty: false,
        }
    }

    fn add() {}

    fn list() {}

    fn edit() {}

    fn delete() {}
}

impl Drop for Notes {
    fn drop(&mut self) {
        if self.dirty {
            storage::save(FILENAME, &self.notes);
        }
    }
}
