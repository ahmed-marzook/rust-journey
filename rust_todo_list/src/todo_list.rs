use crate::Task;

#[derive(Debug)]
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
    pub fn add_task(&mut self, mut task: Task) -> usize {
        let index = self.tasks.len();
        task.id = Some(self.generate_id());
        self.tasks.push(task);
        println!(
            "Added task {} with id {:?}",
            self.tasks[index].title, self.tasks[index].id
        );
        index
    }

    pub fn list_tasks(&self) {
        for task in &self.tasks {
            println!("• {}: Status = {}", task.title, task.is_completed());
        }
    }

    fn generate_id(&mut self) -> u32 {
        let current_id = self.id_gen;
        self.id_gen += 1;
        current_id
    }

    pub fn mark_task_as_done_by_id(&mut self, id: u32) -> Option<()> {
        let task = self.tasks.iter_mut().find(|task| task.id == Some(id))?;
        task.mark_completed();
        println!("Completed task {}", task.title,);
        Some(())
    }

    pub fn remove_task_by_id(&mut self, id: u32) -> Option<()> {
        let index = self.tasks.iter().position(|task| task.id == Some(id))?;

        let removed_task = self.tasks.remove(index);
        println!("Removed task {}", removed_task.title,);
        Some(())
    }
}
