mod priority;
mod status;
mod task;
// use std::io;
use priority::Priority;
use status::Status;
use task::Task;
use uuid::Uuid;

fn main() {
    let example_task = Task {
        id: Uuid::new_v4(),
        title: String::from("Learn UUIDs"),
        status: Status::Pending,
        priority: Priority::High,
    };

    println!("example_task is {example_task:?}");
}
