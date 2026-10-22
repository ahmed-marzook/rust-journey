mod priority;
mod status;
mod task;
mod todo_list;
// use std::io;
use priority::Priority;
use status::Status;
use task::Task;
use todo_list::TodoList;
use uuid::Uuid;

fn main() {
    let example_task1 = Task {
        id: Uuid::new_v4(),
        title: String::from("Learn UUIDs"),
        status: Status::Pending,
        priority: Some(Priority::Medium),
        description: None,
    };
    let example_task2 = Task {
        id: Uuid::new_v4(),
        title: String::from("Gym"),
        status: Status::Completed,
        priority: None,
        description: None,
    };
    let example_task3 = Task {
        id: Uuid::new_v4(),
        title: String::from("Learn Rust"),
        status: Status::Pending,
        priority: Some(Priority::High),
        description: None,
    };
    let example_task4 = Task {
        id: Uuid::new_v4(),
        title: String::from("Learn back flip"),
        status: Status::Skipped,
        priority: Some(Priority::Low),
        description: Some(String::from("Learn to do a back flip from the ground")),
    };

    let mut todo_list = TodoList { tasks: Vec::new() };

    todo_list.add_task(example_task1);
    todo_list.add_task(example_task2);
    todo_list.add_task(example_task3);
    todo_list.add_task(example_task4);

    println!("This is the first task {:?}", { &todo_list.tasks[0] });
    let _ = &todo_list.tasks[0].mark_completed();
    println!("This is the first task {:?}", { &todo_list.tasks[0] });

    todo_list.list_tasks();
}
