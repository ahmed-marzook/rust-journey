mod cli;
mod config;
mod dates;
mod db;
mod display;
mod priority;
mod status;
mod task;
mod task_repo;
mod todo_filter;
mod todo_list;
use chrono::Local;
use cli::parse;
use rusqlite::Connection;
use status::Status;
use std::env;
use todo_list::TodoList;

const DB_FILE: &str = "todo.db";

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = parse(&args)?;

    // Reads config.toml, creating it with the default (dummy) user on first run.
    // Done after parsing, so a mistyped command doesn't create any files.
    let config = config::load_or_create(config::CONFIG_FILE)?;

    // Opens todo.db, creating it and/or upgrading its schema if needed.
    let conn = db::open(DB_FILE)?;

    match command {
        cli::Command::Todo(command) => match command {
            cli::TodoCommand::Add {
                title,
                priority,
                due_date,
            } => {
                task_repo::add(&conn, &title, priority, due_date)?;
                println!("Added \"{title}\"");
            }
            cli::TodoCommand::List { filter } => {
                // Work out today once, in the user's timezone, and share it with
                // both steps so they agree on what "overdue" means.
                let today = Local::now().date_naive();
                let todo_list = TodoList::new(task_repo::all(&conn)?);
                // todo_list picks the tasks (data), display prints them (screen).
                let tasks = todo_list.visible_tasks(filter, today);
                display::print_tasks(&tasks, today);
            }
            // `?` sends any error up to main(), which prints it and exits with code 1.
            // Each task_repo call IS the save: once it returns Ok, the change is
            // in the database, so the message after it is always true.
            // Messages show the title, not a number: numbers can shift after any change.
            cli::TodoCommand::Done { id } => {
                let id = id_for_number(&conn, id)?;
                let title = task_repo::set_status(&conn, id, Status::Completed)?;
                println!("Completed \"{title}\"");
            }
            cli::TodoCommand::Remove { id } => {
                let id = id_for_number(&conn, id)?;
                let title = task_repo::remove(&conn, id)?;
                println!("Removed \"{title}\"");
            }
            cli::TodoCommand::Priority { id, priority } => {
                let id = id_for_number(&conn, id)?;
                let title = task_repo::set_priority(&conn, id, priority)?;

                match priority {
                    Some(p) => println!("Set priority of \"{title}\" to {p}"),
                    None => println!("Cleared priority of \"{title}\""),
                }
            }
            cli::TodoCommand::DueDate { id, due_date } => {
                let id = id_for_number(&conn, id)?;
                let title = task_repo::set_due_date(&conn, id, due_date)?;

                match due_date {
                    Some(d) => println!("Set due date of \"{title}\" to {}", d.format("%d-%m-%Y")),
                    None => println!("Cleared due date of \"{title}\""),
                }
            }
            cli::TodoCommand::Check => {
                let health = db::check(&conn)?;
                display::print_health(&health, DB_FILE, &config.user);

                // An unhealthy database is an error: exit code 1, so scripts
                // (or a Docker health check, later) can tell something is wrong.
                if !health.is_ok() {
                    return Err("Database check failed".to_string());
                }
                println!("All checks passed");
            }
        },
        // cli::Command::Shop(_) => {
        //     return Err("Shop commands are not implemented yet".to_string());
        // }
    }

    Ok(())
}

// The number shown by `todo list` → the task's permanent id.
// Reads all tasks so it can sort them in exactly the same order as `list`
// (TodoList::sorted_tasks), which is what the numbers refer to.
fn id_for_number(conn: &Connection, number: usize) -> Result<i64, String> {
    TodoList::new(task_repo::all(conn)?).id_for_number(number)
}
