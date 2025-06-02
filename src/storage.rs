use std::fs;

// todo add safety to these

pub(crate) fn load(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).unwrap();
    let notes: Vec<String> = serde_json::from_str(&data).unwrap();

    return notes;
}

pub(crate) fn save(filename: &str, notes: &Vec<String>) {
    let json = serde_json::to_string(notes).unwrap();
    fs::write(filename, json.as_bytes()).unwrap();
}
