mod priority;
mod status;
mod task;
mod todo_list;
mod cli;
mod storage;
// use priority::Priority;
use todo_list::TodoList;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut todo_list = storage::load("todo.json")?;
    let args: Vec<String> = env::args().skip(1).collect();
    // let command = &args[1];

    // match cli::parse(&args) {
    //     Ok(command) => {
    //         // Match on command and run the appropriate operation.
    //     }
    //     Err(error) => eprintln!("{error}"),
    // }

    // print!("{}", &args[0]);

    todo_list.add_task(String::from("Go Gym"), None, None);

    todo_list.list_tasks();

    storage::save(&todo_list, "todo.json")?;

    Ok(())
}
