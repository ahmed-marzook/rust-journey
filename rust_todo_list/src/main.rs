mod priority;
mod status;
mod task;
// use std::io;
use priority::Priority;
use status::Status;
use task::Task;
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

    let mut tasks: Vec<Task> = vec![example_task1, example_task2, example_task3, example_task4];

    tasks.push(Task {
        id: Uuid::new_v4(),
        title: String::from("Practice Claude Cert"),
        status: Status::Pending,
        priority: None,
        description: None,
    });

    println!("{tasks:#?}");

    println!("This is the first task {:?}", { &tasks[0] });

    for task in &tasks {
        println!("{}: completed = {}", task.title, task.is_completed());
    }
}
