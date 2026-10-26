mod cli;
mod priority;
mod status;
mod storage;
mod task;
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
            cli::TodoCommand::Add { title } => {
                todo_list.add_task(title, None, None);
                storage::save(&todo_list, TODO_FILE)?;
            }
            cli::TodoCommand::List => {
                todo_list.list_tasks();
            }
            cli::TodoCommand::Done { id } => {
                // One way of handling Result
                match todo_list.mark_task_as_done_by_id(id) {
                    Ok(_) => {
                        storage::save(&todo_list, TODO_FILE)?;
                    }
                    Err(error) => eprintln!("{error}"),
                }
            }
            cli::TodoCommand::Remove { id } => {
                // second way of handlign result
                if let Err(error) = todo_list.remove_task_by_id(id) {
                    eprintln!("{error}");
                } else {
                    storage::save(&todo_list, TODO_FILE)?;
                }
            }
            cli::TodoCommand::Priority { id, priority } => {
                // second way of handlign result
                if let Err(error) = todo_list.set_priority_by_id(id, priority.clone()) {
                    eprintln!("{error}");
                } else {
                    storage::save(&todo_list, TODO_FILE)?;

                    match &priority {
                        Some(p) => println!("Set priority of task {id} to {p:?}"),
                        None => println!("Cleared priority of task {id}"),
                    }
                }
            }
        },
        // cli::Command::Shop(_) => {
        //     return Err("Shop commands are not implemented yet".to_string());
        // }
    }

    Ok(())
}
