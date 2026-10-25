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

pub fn load(path: &str) -> Result<TodoList, String> {
    // file doesnt exist
    if !Path::new(path).exists() {
        // Start with an empty list.
        let list = TodoList::new();

        // Write it to disk straight away, so todo.json now exists.
        // We reuse our own save function. Its error is already a String,
        // so `?` can pass it straight up with no map_err needed.
        save(&list, path)?;

        // `return` exits early so we skip the "read the file" code below.
        return Ok(list);
    }

    // The file exists, so read all of it into a String.
    //    read_to_string returns Result<String, std::io::Error>.
    let contents = fs::read_to_string(path).map_err(|e| format!("Failed to read {path}: {e}"))?;

    // 3. Turn the JSON text back into a TodoList.
    //    from_str is generic: it can produce any Deserialize type.
    //    How does it know we want a TodoList? From the type annotation
    //    on `list`. Rust works it out from what we said we want.
    let list: TodoList =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse {path}: {e}"))?;

    Ok(list)
}
