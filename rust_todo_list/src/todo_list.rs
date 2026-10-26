use crate::status::Status;
use crate::task::Task;
use crate::{priority::Priority, todo_filter::TodoFilter};
use chrono::{NaiveDate, Utc};
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

    pub fn id_for_number(&self, number: usize) -> Result<u32, String> {
        let index = number
            .checked_sub(1)
            .ok_or_else(|| "Task number must be at least 1".to_string())?;
        let tasks = self.sorted_tasks();
        tasks
            .get(index)
            .map(|task| task.id)
            .ok_or_else(|| format!("No task #{number}, there are {} tasks", tasks.len()))
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

    fn sorted_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();
        // Default order. The key is a tuple, and tuples compare left to right:
        // each item only matters when every item before it is equal.
        // This sorts the Vec of references, so the order saved in todo.json is untouched.
        tasks.sort_by_key(|task| {
            (
                // 1. Pending before completed: false sorts before true.
                task.status == Status::Completed,
                // 2. Tasks with a due date first. Rust sorts None before Some,
                //    so compare "has no date?" first: false (has one) comes first.
                task.due_date.is_none(),
                // 3. Then earliest date first, which puts overdue tasks at the top.
                task.due_date,
                // 4. Tasks with a priority before ones without (same None trick).
                task.priority.is_none(),
                // 5. Then High → Medium → Low: the order the variants are declared
                //    in, thanks to the Ord derive on Priority.
                task.priority,
                // 6. Tie-breaker: oldest task first.
                task.created_at,
            )
        });
        tasks
    }

    // Returns the tasks to show, each paired with its index in the full sorted list.
    // Doesn't print anything: display.rs does that, so this can be tested on its own.
    // `today` is passed in (not read from the clock here) for the same reason.
    pub fn visible_tasks(
        &self,
        todo_filter: Option<TodoFilter>,
        today: NaiveDate,
    ) -> Vec<(usize, &Task)> {
        // Sort all tasks, number them, then keep only the ones that pass the filter.
        self.sorted_tasks()
            .into_iter()
            // Number the tasks BEFORE filtering, so each task keeps its position
            // in the full sorted list. Commands like `todo done 4` look the number
            // up in that full list (they don't know which filter was used), so a
            // filtered view must show the same numbers, gaps included.
            // Moving this after .filter() would renumber each view from 1, and
            // `done` could then silently complete the wrong task.
            .enumerate()
            // is_none_or: no filter at all → keep every task,
            //             a filter → keep only the tasks it matches.
            .filter(|(_, task)| todo_filter.is_none_or(|f| f.matches(task, today)))
            .collect()
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
