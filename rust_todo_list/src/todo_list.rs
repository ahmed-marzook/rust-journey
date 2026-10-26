use crate::status::Status;
use crate::task::Task;
use crate::{priority::Priority, todo_filter::TodoFilter};
use chrono::{Local, NaiveDate, Utc};
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
        due_date: Option<NaiveDate>,
    ) -> u32 {
        let id = self.generate_id();
        let now = Utc::now();
        let task = Task {
            id,
            title,
            status: Status::Pending,
            priority,
            due_date: due_date,
            created_at: now,
            updated_at: now,
        };
        println!("Added task {} with id {}", task.title, id);
        self.tasks.push(task);
        id
    }

    pub fn list_tasks(&self, todo_filter: Option<TodoFilter>) {
        // Work out today once, in the user's timezone (needs `use chrono::Local;`)
        let today = Local::now().date_naive();

        // Keep only the tasks that pass the filter.
        let tasks: Vec<&Task> = self
            .tasks
            .iter()
            // is_none_or: no filter at all → keep every task,
            //             a filter → keep only the tasks it matches.
            .filter(|task| todo_filter.is_none_or(|f| f.matches(task, today)))
            .collect();

        if tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        // Header, using the SAME widths as the rows below so the columns line up.
        //   {:>3}  right-aligned, 3 wide  (numbers look best right-aligned)
        //   {:<4}  left-aligned, 4 wide
        println!(
            "{:>3}  {:<4}  {:<8}  {:<10}  {}",
            "ID", "Done", "Priority", "Due", "Title"
        );
        println!("{}", "-".repeat(50)); // a divider line: "-" repeated 50 times

        for task in tasks {
            // A small checkbox for the status.
            // Every variant needs an arm, so Skipped gets one too.
            let done = match task.status {
                Status::Completed => "[x]",
                Status::Skipped => "[-]",
                Status::Pending => "[ ]",
            };

            // Option<Priority> → text: the priority's name, or "-" if it has none.
            // .map turns Some(High) into Some("High"); unwrap_or_else supplies "-" for None.
            let priority = task
                .priority
                .map(|p| p.to_string())
                .unwrap_or_else(|| "-".to_string());

            // Same idea for the due date, formatted like the input: dd-mm-yyyy.
            let due = task
                .due_date
                .map(|d| d.format("%d-%m-%Y").to_string())
                .unwrap_or_else(|| "-".to_string());

            // Reuse the filter logic to mark overdue tasks, so there's only ONE
            // definition of "overdue" in the whole program.
            let overdue = if TodoFilter::OverDue.matches(task, today) {
                "  (overdue)"
            } else {
                ""
            };

            println!(
                "{:>3}  {:<4}  {:<8}  {:<10}  {}{}",
                task.id, done, priority, due, task.title, overdue
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

    pub fn set_priority_by_id(
        &mut self,
        id: u32,
        priority: Option<Priority>,
    ) -> Result<(), String> {
        let index = self
            .find_index(id)
            .ok_or_else(|| format!("No task found with ID {id}"))?;

        self.tasks[index].priority = priority;
        self.tasks[index].updated_at = Utc::now();

        Ok(())
    }

    pub fn set_due_date_by_id(
        &mut self,
        id: u32,
        due_date: Option<NaiveDate>,
    ) -> Result<(), String> {
        let index = self
            .find_index(id)
            .ok_or_else(|| format!("No task found with ID {id}"))?;

        self.tasks[index].due_date = due_date;
        self.tasks[index].updated_at = Utc::now();

        Ok(())
    }
}
