mod cli;
mod priority;
mod status;
mod storage;
mod task;
mod todo_filter;
mod todo_list;
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
                todo_list.add_task(title, priority, due_date);
                storage::save(&todo_list, TODO_FILE)?;
            }
            cli::TodoCommand::List { filter } => {
                todo_list.list_tasks(filter);
            }
            // `?` sends any error up to main(), which prints it and exits with code 1.
            // If the change fails, the save line is never reached.
            cli::TodoCommand::Done { id } => {
                todo_list.mark_task_as_done_by_id(id)?;
                storage::save(&todo_list, TODO_FILE)?;
            }
            cli::TodoCommand::Remove { id } => {
                todo_list.remove_task_by_id(id)?;
                storage::save(&todo_list, TODO_FILE)?;
            }
            cli::TodoCommand::Priority { id, priority } => {
                todo_list.set_priority_by_id(id, priority)?;
                storage::save(&todo_list, TODO_FILE)?;

                match priority {
                    Some(p) => println!("Set priority of task {id} to {p}"),
                    None => println!("Cleared priority of task {id}"),
                }
            }
            cli::TodoCommand::DueDate { id, due_date } => {
                todo_list.set_due_date_by_id(id, due_date)?;
                storage::save(&todo_list, TODO_FILE)?;

                match due_date {
                    Some(d) => println!("Set due date of task {id} to {}", d.format("%d-%m-%Y")),
                    None => println!("Cleared due date of task {id}"),
                }
            }
        },
        // cli::Command::Shop(_) => {
        //     return Err("Shop commands are not implemented yet".to_string());
        // }
    }

    Ok(())
}
