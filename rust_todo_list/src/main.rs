mod cli;
mod dates;
mod display;
mod priority;
mod status;
mod storage;
mod task;
mod todo_filter;
mod todo_list;
use chrono::Local;
use cli::parse;
use std::env;

const TODO_FILE: &str = "todo.json";

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = parse(&args)?;

    let mut todo_list = storage::load(TODO_FILE)?;

    match command {
        cli::Command::Todo(command) => match command {
            cli::TodoCommand::Add {
                title,
                priority,
                due_date,
            } => {
                // Build the message first: format! only borrows `title`,
                // and add_task then takes ownership of it.
                let message = format!("Added \"{title}\"");
                todo_list.add_task(title, priority, due_date);
                storage::save(&todo_list, TODO_FILE)?;
                println!("{message}");
            }
            cli::TodoCommand::List { filter } => {
                // Work out today once, in the user's timezone, and share it with
                // both steps so they agree on what "overdue" means.
                let today = Local::now().date_naive();
                // todo_list picks the tasks (data), display prints them (screen).
                let tasks = todo_list.visible_tasks(filter, today);
                display::print_tasks(&tasks, today);
            }
            // `?` sends any error up to main(), which prints it and exits with code 1.
            // If the change fails, the save line is never reached.
            // Messages are printed only AFTER the save succeeds, so we never
            // report a change that wasn't actually written to disk.
            // They show the title, not a number: numbers can shift after any change.
            cli::TodoCommand::Done { id } => {
                let id = todo_list.id_for_number(id)?;
                let title = todo_list.mark_task_as_done_by_id(id)?;
                storage::save(&todo_list, TODO_FILE)?;
                println!("Completed \"{title}\"");
            }
            cli::TodoCommand::Remove { id } => {
                let id = todo_list.id_for_number(id)?;
                let removed = todo_list.remove_task_by_id(id)?;
                storage::save(&todo_list, TODO_FILE)?;
                println!("Removed \"{}\"", removed.title);
            }
            cli::TodoCommand::Priority { id, priority } => {
                let id = todo_list.id_for_number(id)?;
                let title = todo_list.set_priority_by_id(id, priority)?;
                storage::save(&todo_list, TODO_FILE)?;

                match priority {
                    Some(p) => println!("Set priority of \"{title}\" to {p}"),
                    None => println!("Cleared priority of \"{title}\""),
                }
            }
            cli::TodoCommand::DueDate { id, due_date } => {
                let id = todo_list.id_for_number(id)?;
                let title = todo_list.set_due_date_by_id(id, due_date)?;
                storage::save(&todo_list, TODO_FILE)?;

                match due_date {
                    Some(d) => println!("Set due date of \"{title}\" to {}", d.format("%d-%m-%Y")),
                    None => println!("Cleared due date of \"{title}\""),
                }
            }
        },
        // cli::Command::Shop(_) => {
        //     return Err("Shop commands are not implemented yet".to_string());
        // }
    }

    Ok(())
}
