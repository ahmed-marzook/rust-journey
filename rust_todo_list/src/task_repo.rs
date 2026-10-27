// All SQL for tasks lives here. Each function is one small query that
// changes only the row it touches: there is no "load everything, change it,
// save everything" any more. The connection comes from db::open(), which has
// already made sure the tasks table exists with the latest schema.

use crate::priority::Priority;
use crate::status::Status;
use crate::task::Task;
use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::{Connection, params};

// Adds a new pending task and returns the id SQLite gave it.
pub fn add(
    conn: &Connection,
    title: &str,
    priority: Option<Priority>,
    due_date: Option<NaiveDate>,
) -> Result<i64, String> {
    let now = Utc::now();
    // ?1, ?2 ... are placeholders, filled from params![] in order.
    // Never build SQL with format!: placeholders keep values separate
    // from the SQL itself, which prevents SQL injection.
    conn.execute(
        "INSERT INTO tasks (title, status, priority, due_date, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        params![
            title,
            Status::Pending.as_db_str(),
            priority.map(|p| p.as_db_str()), // None → NULL
            due_date,                        // None → NULL, Some → 'YYYY-MM-DD'
            now,                             // ?5 is used twice: created_at and updated_at
        ],
    )
    .map_err(|e| format!("Failed to add task: {e}"))?;

    // No id was given in the INSERT, so SQLite picked one (AUTOINCREMENT).
    Ok(conn.last_insert_rowid())
}

// Reads every task. Sorting, numbering and filtering happen in Rust
// (TodoList), so the order used for numbers is defined in ONE place.
pub fn all(conn: &Connection) -> Result<Vec<Task>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, title, status, priority, due_date, created_at, updated_at
             FROM tasks",
        )
        .map_err(|e| format!("Failed to read tasks: {e}"))?;

    // query_map runs the query and calls the closure once per row.
    // row.get(n) reads column n and converts it to the type we ask for.
    // Status and Priority are read as plain text here, and turned into
    // enums below, because their errors are Strings, not rusqlite errors.
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?, // NULL → None
                row.get::<_, Option<NaiveDate>>(4)?,
                row.get::<_, DateTime<Utc>>(5)?,
                row.get::<_, DateTime<Utc>>(6)?,
            ))
        })
        .map_err(|e| format!("Failed to read tasks: {e}"))?;

    let mut tasks = Vec::new();
    for row in rows {
        let (id, title, status, priority, due_date, created_at, updated_at) =
            row.map_err(|e| format!("Failed to read a task: {e}"))?;

        tasks.push(Task {
            id,
            title,
            // The same FromStr you use for the command line.
            status: status.parse::<Status>()?,
            // Option<String> → Option<Priority>. transpose() flips
            // Option<Result<..>> into Result<Option<..>> so ? can be used.
            priority: priority.map(|p| p.parse::<Priority>()).transpose()?,
            due_date,
            created_at,
            updated_at,
        });
    }
    Ok(tasks)
}

// The functions below change one task and return its title, so main.rs can
// print a message like `Completed "Gym"`.
//
// RETURNING title: SQLite sends back the title of the row it just changed,
// in the same statement. If no row matched the id, there is nothing to
// return, so query_row gives QueryReturnedNoRows, which not_found() turns
// into a friendly "No task found" message. One statement means there's no
// gap between "does it exist?" and "change it".

pub fn set_status(conn: &Connection, id: i64, status: Status) -> Result<String, String> {
    conn.query_row(
        "UPDATE tasks SET status = ?1, updated_at = ?2 WHERE id = ?3 RETURNING title",
        params![status.as_db_str(), Utc::now(), id],
        |row| row.get(0),
    )
    .map_err(|e| not_found(e, id))
}

pub fn set_priority(
    conn: &Connection,
    id: i64,
    priority: Option<Priority>,
) -> Result<String, String> {
    conn.query_row(
        "UPDATE tasks SET priority = ?1, updated_at = ?2 WHERE id = ?3 RETURNING title",
        params![priority.map(|p| p.as_db_str()), Utc::now(), id],
        |row| row.get(0),
    )
    .map_err(|e| not_found(e, id))
}

pub fn set_due_date(
    conn: &Connection,
    id: i64,
    due_date: Option<NaiveDate>,
) -> Result<String, String> {
    conn.query_row(
        "UPDATE tasks SET due_date = ?1, updated_at = ?2 WHERE id = ?3 RETURNING title",
        params![due_date, Utc::now(), id],
        |row| row.get(0),
    )
    .map_err(|e| not_found(e, id))
}

pub fn remove(conn: &Connection, id: i64) -> Result<String, String> {
    conn.query_row(
        "DELETE FROM tasks WHERE id = ?1 RETURNING title",
        params![id],
        |row| row.get(0),
    )
    .map_err(|e| not_found(e, id))
}

// Turns a rusqlite error into our String error. "No row came back" means
// the id didn't exist; anything else is a real database problem.
fn not_found(error: rusqlite::Error, id: i64) -> String {
    match error {
        rusqlite::Error::QueryReturnedNoRows => format!("No task found with ID {id}"),
        other => format!("Database error: {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    // A fresh, fully migrated database in memory for each test.
    fn test_db() -> Connection {
        db::open(":memory:").unwrap()
    }

    #[test]
    fn add_then_read_back() {
        let conn = test_db();
        let due = NaiveDate::from_ymd_opt(2026, 10, 30);
        let id = add(&conn, "Gym", Some(Priority::High), due).unwrap();
        add(&conn, "Read", None, None).unwrap();

        let tasks = all(&conn).unwrap();
        assert_eq!(tasks.len(), 2);

        let gym = tasks.iter().find(|t| t.id == id).unwrap();
        assert_eq!(gym.title, "Gym");
        assert_eq!(gym.status, Status::Pending);
        assert_eq!(gym.priority, Some(Priority::High));
        assert_eq!(gym.due_date, due);
    }

    #[test]
    fn set_status_marks_done_and_returns_title() {
        let conn = test_db();
        let id = add(&conn, "Gym", None, None).unwrap();

        assert_eq!(set_status(&conn, id, Status::Completed).unwrap(), "Gym");
        assert_eq!(all(&conn).unwrap()[0].status, Status::Completed);
    }

    #[test]
    fn priority_can_be_set_and_cleared() {
        let conn = test_db();
        let id = add(&conn, "Gym", None, None).unwrap();

        set_priority(&conn, id, Some(Priority::Low)).unwrap();
        assert_eq!(all(&conn).unwrap()[0].priority, Some(Priority::Low));

        set_priority(&conn, id, None).unwrap();
        assert_eq!(all(&conn).unwrap()[0].priority, None);
    }

    #[test]
    fn due_date_can_be_set_and_cleared() {
        let conn = test_db();
        let id = add(&conn, "Gym", None, None).unwrap();
        let due = NaiveDate::from_ymd_opt(2026, 12, 1);

        set_due_date(&conn, id, due).unwrap();
        assert_eq!(all(&conn).unwrap()[0].due_date, due);

        set_due_date(&conn, id, None).unwrap();
        assert_eq!(all(&conn).unwrap()[0].due_date, None);
    }

    #[test]
    fn remove_deletes_and_returns_title() {
        let conn = test_db();
        let id = add(&conn, "Gym", None, None).unwrap();

        assert_eq!(remove(&conn, id).unwrap(), "Gym");
        assert!(all(&conn).unwrap().is_empty());
    }

    #[test]
    fn missing_id_gives_not_found() {
        let conn = test_db();
        let expected = "No task found with ID 99";
        assert_eq!(set_status(&conn, 99, Status::Completed).unwrap_err(), expected);
        assert_eq!(set_priority(&conn, 99, None).unwrap_err(), expected);
        assert_eq!(set_due_date(&conn, 99, None).unwrap_err(), expected);
        assert_eq!(remove(&conn, 99).unwrap_err(), expected);
    }
}
