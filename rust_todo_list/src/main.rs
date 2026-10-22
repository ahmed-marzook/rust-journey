mod priority;
mod status;
mod task;
mod todo_list;
use priority::Priority;
use todo_list::TodoList;

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_task(String::from("Learn UUIDs"), Some(Priority::Medium), None);
    todo_list.add_task(String::from("Gym"), None, None);
    todo_list.add_task(String::from("Learn Rust"), Some(Priority::High), None);
    todo_list.add_task(
        String::from("Learn back flip"),
        Some(Priority::Low),
        Some(String::from("Learn to do a back flip from the ground"))
    );

    todo_list.list_tasks();

    if let Err(error) = todo_list.mark_task_as_done_by_id(2) {
        println!("{error}");
    }

    if let Err(error) = todo_list.mark_task_as_done_by_id(99) {
        println!("{error}");
    }

    match todo_list.remove_task_by_id(2) {
        Some(task) => println!("Removed {}", task.title),
        None => println!("No task with id 2"),
    }

    todo_list.list_tasks();
}
