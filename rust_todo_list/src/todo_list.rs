use crate::priority::Priority;
use crate::status::Status;
use crate::task::Task;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub tasks: Vec<Task>,
    id_gen: u32,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            id_gen: 1,
        }
    }
    pub fn add_task(
        &mut self,
        title: String,
        priority: Option<Priority>,
        description: Option<String>
    ) -> u32 {
        let id = self.generate_id();
        let task = Task {
            id: id,
            title,
            status: Status::Pending,
            priority,
            description,
        };
        println!("Added task {} with id {}", task.title, id);
        self.tasks.push(task);
        id
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{:?}] {} — {:?} {:?} {:?}",
                task.id,
                task.title,
                task.status,
                task.priority,
                task.description
            );
        }
    }

    fn generate_id(&mut self) -> u32 {
        let current_id = self.id_gen;
        self.id_gen += 1;
        current_id
    }

    pub fn mark_task_as_done_by_id(&mut self, id: u32) -> Result<(), String> {
        let task = self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| format!("No task found with ID {id}"))?;

        task.mark_completed();
        println!("Completed task {}", task.title);

        Ok(())
    }

    pub fn remove_task_by_id(&mut self, id: u32) -> Option<Task> {
        let index = self.tasks.iter().position(|task| task.id == id)?;

        let removed_task = self.tasks.remove(index);
        println!("Removed task {}", removed_task.title);
        Some(removed_task)
    }
}
