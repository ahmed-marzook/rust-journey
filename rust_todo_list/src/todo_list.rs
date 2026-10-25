use crate::priority::Priority;
use crate::status::Status;
use crate::task::Task;
use chrono::Utc;
use serde::{Deserialize, Serialize};

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

    fn generate_id(&mut self) -> u32 {
        let current_id = self.id_gen;
        self.id_gen += 1;
        current_id
    }

    fn find_index(&self, id: u32) -> Option<usize> {
        self.tasks.iter().position(|task| task.id == id)
    }

    pub fn add_task(
        &mut self,
        title: String,
        priority: Option<Priority>,
        description: Option<String>,
    ) -> u32 {
        let id = self.generate_id();
        let now = Utc::now();
        let task = Task {
            id,
            title,
            status: Status::Pending,
            priority,
            description,
            created_at: now,
            updated_at: now,
        };
        println!("Added task {} with id {}", task.title, id);
        self.tasks.push(task);
        id
    }

    pub fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!(
                "{}) ID: [{}] {} — {:?} {:?} {:?} {}",
                index,
                task.id,
                task.title,
                task.status,
                task.priority,
                task.description,
                task.created_at.format("%d-%m-%Y %H:%M")
            );
        }
    }

    pub fn mark_task_as_done_by_id(&mut self, id: u32) -> Result<(), String> {
        let index = self
            .find_index(id)
            .ok_or_else(|| format!("No task found with ID {id}"))?;

        self.tasks[index].mark_completed();
        println!("Completed task {}", self.tasks[index].title);

        Ok(())
    }

    pub fn remove_task_by_id(&mut self, id: u32) -> Result<Task, String> {
        let index = self
            .find_index(id)
            .ok_or_else(|| format!("No task found with ID {id}"))?;

        let removed_task = self.tasks.remove(index);
        println!("Removed task {}", removed_task.title);
        Ok(removed_task)
    }
}
