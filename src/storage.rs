use std::{fs::File, io::Write};

pub(crate) fn load(filename: &str) -> Vec<String> {
    // todo load and return
    todo!()
}

pub(crate) fn save(filename: &str, notes: &Vec<String>) {
    // todo is unwrap here unsafe
    let json = serde_json::to_string(notes).unwrap();

    // todo is unwrap unsafe here too
    let mut file = File::create(filename).unwrap();
    // todo this could error
    file.write_all(json.as_bytes());
}
