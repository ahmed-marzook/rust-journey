mod priority;
mod status;
mod task;
mod todo_list;
use priority::Priority;
use status::Status;
use task::Task;
use todo_list::TodoList;

fn main() {
    let example_task1 = Task {
        id: None,
        title: String::from("Learn UUIDs"),
        status: Status::Pending,
        priority: Some(Priority::Medium),
        description: None,
    };
    let example_task2 = Task {
        id: None,
        title: String::from("Gym"),
        status: Status::Completed,
        priority: None,
        description: None,
    };
    let example_task3 = Task {
        id: None,
        title: String::from("Learn Rust"),
        status: Status::Pending,
        priority: Some(Priority::High),
        description: None,
    };
    let example_task4 = Task {
        id: None,
        title: String::from("Learn back flip"),
        status: Status::Skipped,
        priority: Some(Priority::Low),
        description: Some(String::from("Learn to do a back flip from the ground")),
    };

    let mut todo_list = TodoList::new();

    todo_list.add_task(example_task1);
    todo_list.add_task(example_task2);
    todo_list.add_task(example_task3);
    todo_list.add_task(example_task4);

    todo_list.list_tasks();

    if let Err(error) = todo_list.mark_task_as_done_by_id(2) {
        println!("{error}");
    }

    todo_list.remove_task_by_id(2);

    todo_list.list_tasks();
}
