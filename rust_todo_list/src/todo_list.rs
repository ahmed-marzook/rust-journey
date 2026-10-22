use crate::Task;

#[derive(Debug)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn add_task(&mut self, task: Task) -> usize {
        let index = self.tasks.len();
        self.tasks.push(task);
        println!("Added task at index {index}");
        index
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("• {}: Status = {}", task.title, task.is_completed());
        }
    }
}
