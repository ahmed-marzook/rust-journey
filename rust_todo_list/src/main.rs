mod priority;
mod status;
mod task;
mod todo_list;
use priority::Priority;
use todo_list::TodoList;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    dbg!(args);
    let mut todo_list = TodoList::new();

    todo_list.list_tasks();
}
