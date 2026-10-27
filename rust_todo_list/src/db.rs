// Everything about creating, upgrading and checking the SQLite database.
// The rest of the program only calls db::open() and gets back a connection
// that is guaranteed to have the latest schema.

use rusqlite::Connection;

// Each entry is one schema change, applied once, in order.
// The database remembers how many have been applied (PRAGMA user_version),
// so on startup only the missing ones run.
//
// Rules: never edit or reorder an entry once it has been used on a real
// database. To change the schema, ADD a new entry at the end
// (e.g. "ALTER TABLE tasks ADD COLUMN notes TEXT;").
const MIGRATIONS: &[&str] = &[
    // 1: initial schema
    "CREATE TABLE tasks (
        -- AUTOINCREMENT: SQLite picks the id and never reuses one,
        -- even after the newest task is deleted.
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        title       TEXT    NOT NULL CHECK (length(title) > 0),
        status      TEXT    NOT NULL CHECK (status IN ('pending', 'completed', 'skipped')),
        -- NULL means no priority; CHECK still allows NULL.
        priority    TEXT             CHECK (priority IN ('high', 'medium', 'low')),
        -- Stored as 'YYYY-MM-DD'. date() returns NULL for text that isn't a
        -- date ('tmr'), but ROLLS OVER impossible days ('2026-02-31' becomes
        -- '2026-03-03'). Requiring date() to give back the exact same text
        -- rejects both, and also any other format like '30-10-2026'.
        -- IS, not =: with '=', NULL = 'tmr' gives NULL (unknown), and CHECK
        -- only rejects a row when the result is false, so 'tmr' would get in.
        -- IS treats NULL as a normal value, so it gives a plain false.
        due_date    TEXT             CHECK (due_date IS NULL OR date(due_date) IS due_date),
        created_at  TEXT    NOT NULL,
        updated_at  TEXT    NOT NULL
    );",
];

// Opens the database at `path`, creating the file if it doesn't exist,
// then brings its schema up to date. Safe to call on every run.
// Pass ":memory:" for a throwaway in-memory database (used by the tests).
pub fn open(path: &str) -> Result<Connection, String> {
    let mut conn = Connection::open(path).map_err(|e| format!("Failed to open {path}: {e}"))?;
    configure(&conn)?;
    migrate(&mut conn)?;
    Ok(conn)
}

// Settings that SQLite keeps per connection (not in the file),
// so they must be set every time a connection is opened.
fn configure(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "-- Enforce FOREIGN KEY constraints (off by default in SQLite).
         -- Not needed yet, but it will be once there is a second table.
         PRAGMA foreign_keys = ON;",
    )
    .map_err(|e| format!("Failed to configure database: {e}"))
}

// Which migration the database is at. 0 for a brand-new file.
fn schema_version(conn: &Connection) -> Result<usize, String> {
    // PRAGMA user_version is a number SQLite stores in the file header
    // and otherwise ignores: free for apps to use exactly like this.
    let version: u32 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|e| format!("Failed to read schema version: {e}"))?;
    Ok(version as usize)
}

// Applies any migrations the database hasn't had yet.
fn migrate(conn: &mut Connection) -> Result<(), String> {
    let current = schema_version(conn)?;
    let latest = MIGRATIONS.len();

    // The file is NEWER than this program (e.g. an old build run against a
    // database a newer build already upgraded). Stop rather than risk
    // misreading or damaging it.
    if current > latest {
        return Err(format!(
            "Database schema is version {current}, but this program only knows up to \
             version {latest}. Was it created by a newer version?"
        ));
    }

    // Already up to date: the normal case on every run after the first.
    if current == latest {
        return Ok(());
    }

    // All pending migrations, plus the version bump, in ONE transaction:
    // if any of them fails, none are kept and the version stays where it was,
    // so the next run simply tries again from the same point.
    let tx = conn
        .transaction()
        .map_err(|e| format!("Failed to start migration: {e}"))?;

    // skip(current): migrations 1..=current are already applied.
    // enumerate gives a 0-based index, so i + 1 is the migration's number.
    for (i, sql) in MIGRATIONS.iter().enumerate().skip(current) {
        tx.execute_batch(sql)
            .map_err(|e| format!("Migration {} failed: {e}", i + 1))?;
    }

    // PRAGMA statements can't take ?1 placeholders, so the number is
    // formatted into the SQL. That's safe here: it's our own usize,
    // never text typed by a user.
    tx.execute_batch(&format!("PRAGMA user_version = {latest}"))
        .map_err(|e| format!("Failed to update schema version: {e}"))?;

    tx.commit().map_err(|e| format!("Failed to finish migration: {e}"))
}

// Result of a health check, for a `todo check` style command.
#[derive(Debug)]
pub struct Health {
    pub schema_version: usize,
    pub latest_version: usize,
    // "ok" if SQLite found no corruption, otherwise its first problem report.
    pub integrity: String,
    pub task_count: u32,
}

impl Health {
    pub fn is_ok(&self) -> bool {
        self.integrity == "ok" && self.schema_version == self.latest_version
    }
}

// Looks the database over without changing anything.
pub fn check(conn: &Connection) -> Result<Health, String> {
    // integrity_check scans the whole file. It returns a single row "ok"
    // when healthy, or one row per problem; we keep the first row.
    let integrity: String = conn
        .query_row("PRAGMA integrity_check", [], |row| row.get(0))
        .map_err(|e| format!("Integrity check failed to run: {e}"))?;

    let task_count: u32 = conn
        .query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))
        .map_err(|e| format!("Failed to count tasks: {e}"))?;

    Ok(Health {
        schema_version: schema_version(conn)?,
        latest_version: MIGRATIONS.len(),
        integrity,
        task_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // A fresh, fully migrated database in memory: nothing to clean up.
    fn test_db() -> Connection {
        open(":memory:").unwrap()
    }

    // Inserts one task with the given values. Returns the SQLite result,
    // so tests can check whether the CHECK constraints accepted it.
    fn insert(
        conn: &Connection,
        title: &str,
        status: &str,
        priority: Option<&str>,
        due_date: Option<&str>,
    ) -> rusqlite::Result<usize> {
        conn.execute(
            "INSERT INTO tasks (title, status, priority, due_date, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, '2026-10-27 10:00:00+00:00', '2026-10-27 10:00:00+00:00')",
            rusqlite::params![title, status, priority, due_date],
        )
    }

    #[test]
    fn new_database_is_at_latest_version() {
        let conn = test_db();
        assert_eq!(schema_version(&conn).unwrap(), MIGRATIONS.len());
    }

    #[test]
    fn migrating_twice_is_harmless() {
        let mut conn = test_db();
        // open() already migrated once; a second run must be a no-op.
        migrate(&mut conn).unwrap();
        assert_eq!(schema_version(&conn).unwrap(), MIGRATIONS.len());
    }

    #[test]
    fn refuses_a_database_from_a_newer_version() {
        let mut conn = test_db();
        conn.execute_batch("PRAGMA user_version = 99").unwrap();
        assert!(migrate(&mut conn).is_err());
    }

    #[test]
    fn accepts_a_valid_task() {
        let conn = test_db();
        assert!(insert(&conn, "Gym", "pending", Some("high"), Some("2026-10-30")).is_ok());
        assert!(insert(&conn, "Read", "completed", None, None).is_ok());
    }

    #[test]
    fn rejects_bad_values() {
        let conn = test_db();
        assert!(insert(&conn, "", "pending", None, None).is_err(), "empty title");
        assert!(insert(&conn, "A", "maybe", None, None).is_err(), "unknown status");
        assert!(insert(&conn, "A", "pending", Some("urgent"), None).is_err(), "unknown priority");
        assert!(insert(&conn, "A", "pending", None, Some("2026-02-31")).is_err(), "impossible date");
        assert!(insert(&conn, "A", "pending", None, Some("tmr")).is_err(), "not a date");
    }

    #[test]
    fn ids_are_never_reused() {
        let conn = test_db();
        insert(&conn, "A", "pending", None, None).unwrap();
        insert(&conn, "B", "pending", None, None).unwrap();
        let b = conn.last_insert_rowid();
        conn.execute("DELETE FROM tasks WHERE id = ?1", [b]).unwrap();

        insert(&conn, "C", "pending", None, None).unwrap();
        // Without AUTOINCREMENT, C would get B's old id.
        assert_eq!(conn.last_insert_rowid(), b + 1);
    }

    #[test]
    fn check_reports_a_healthy_database() {
        let conn = test_db();
        insert(&conn, "Gym", "pending", None, None).unwrap();

        let health = check(&conn).unwrap();
        assert!(health.is_ok());
        assert_eq!(health.task_count, 1);
    }
}
