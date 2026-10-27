// Reads settings from config.toml. If the file doesn't exist yet, it's
// created with default values first, so there's always a file to edit.
//
// It lives in the current folder, next to todo.db. In Docker that's /data,
// so the config is kept in the same volume as the tasks.

use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;

pub const CONFIG_FILE: &str = "config.toml";

// Written as-is the first time the app runs. Kept as text (rather than
// generated from a Config value) so the comments end up in the file too.
const DEFAULT_CONFIG: &str = r#"# Settings for rust_todo_list. Changes apply from the next command.

# The user the app runs as.
user = "dummy"
"#;

// Deserialize lets the toml crate build this struct straight from the file:
// each field is filled from the key with the same name.
// deny_unknown_fields: a misspelt key (e.g. `usr = ...`) is an error,
// instead of being silently ignored.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub user: String,
}

// Reads the config file, creating it with the defaults if it's missing.
pub fn load_or_create(path: &str) -> Result<Config, String> {
    // Try to read first, and create only on NotFound, the same pattern the
    // old JSON storage used (no separate "does it exist?" check).
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            fs::write(path, DEFAULT_CONFIG)
                .map_err(|e| format!("Failed to create {path}: {e}"))?;
            DEFAULT_CONFIG.to_string()
        }
        Err(e) => return Err(format!("Failed to read {path}: {e}")),
    };

    parse(&text).map_err(|e| format!("Invalid {path}: {e}"))
}

// Turns the file's text into a Config and checks the values make sense.
// Separate from load_or_create so tests can use it without touching files.
fn parse(text: &str) -> Result<Config, String> {
    let config: Config = toml::from_str(text).map_err(|e| e.to_string())?;

    if config.user.trim().is_empty() {
        return Err("`user` must not be empty".to_string());
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_the_dummy_user() {
        let config = parse(DEFAULT_CONFIG).unwrap();
        assert_eq!(config.user, "dummy");
    }

    #[test]
    fn reads_a_custom_user() {
        assert_eq!(parse(r#"user = "ahmed""#).unwrap().user, "ahmed");
    }

    #[test]
    fn rejects_bad_configs() {
        assert!(parse("").is_err(), "missing user");
        assert!(parse(r#"user = """#).is_err(), "empty user");
        assert!(parse(r#"usr = "ahmed""#).is_err(), "misspelt key");
        assert!(parse("user = 42").is_err(), "wrong type");
    }

    #[test]
    fn creates_the_file_when_missing() {
        // A unique file in the system temp folder, so the test never
        // touches a real config.toml.
        let path = std::env::temp_dir().join(format!("todo-config-test-{}.toml", std::process::id()));
        let path = path.to_str().unwrap();
        let _ = fs::remove_file(path);

        let config = load_or_create(path).unwrap();
        assert_eq!(config.user, "dummy");
        // The file now exists with the default contents.
        assert_eq!(fs::read_to_string(path).unwrap(), DEFAULT_CONFIG);

        fs::remove_file(path).unwrap();
    }
}
