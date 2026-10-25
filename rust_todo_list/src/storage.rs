use crate::todo_list::TodoList;
use std::fs;
use std::path::Path;

pub fn save(list: &TodoList, path: &str) -> Result<(), String> {
    // 1. Turn the TodoList into a JSON string.
    let json = serde_json::to_string_pretty(list)
        .map_err(|e| format!("Failed to convert todo list to JSON: {e}"))?;

    fs::write(path, json).map_err(|e| format!("Failed to write {path}: {e}"))?;

    Ok(())
}

// avoid TOCTOU (“time of check to time of use”) by trying tor ead the file immediatyl first then handle failures
pub fn load(path: &str) -> Result<TodoList, String> {
    match std::fs::read_to_string(path) {
        // Case A: the file was read successfully, so parse the JSON
        Ok(contents) => {
            serde_json::from_str(&contents).map_err(|e| format!("Failed to parse {path}: {e}"))
        }

        // Case B: the read failed BECAUSE the file doesn't exist,
        // which is normal on first run, so start with an empty list
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(TodoList::new()),

        // Case C: the read failed for some OTHER reason
        // (no permission, it's a folder, etc.), which is a real error
        Err(e) => Err(format!("Failed to read {path}: {e}")),
    }
}
