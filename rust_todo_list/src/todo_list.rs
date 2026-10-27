// Pure logic over a list of tasks: sorting, numbering and filtering.
// It never touches the database (task_repo.rs does that), so it's easy to
// test with tasks built by hand.

use crate::status::Status;
use crate::task::Task;
use crate::todo_filter::TodoFilter;
use chrono::NaiveDate;

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    // Wraps the tasks read from the database (task_repo::all).
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    // Turns the number the user typed (as shown by `todo list`) into the
    // task's permanent id, which is what the database queries use.
    pub fn id_for_number(&self, number: usize) -> Result<i64, String> {
        let index = number
            .checked_sub(1)
            .ok_or_else(|| "Task number must be at least 1".to_string())?;
        let tasks = self.sorted_tasks();
        tasks
            .get(index)
            .map(|task| task.id)
            .ok_or_else(|| format!("No task #{number}, there are {} tasks", tasks.len()))
    }

    fn sorted_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();
        // Default order. The key is a tuple, and tuples compare left to right:
        // each item only matters when every item before it is equal.
        // This sorts a Vec of references, so the tasks themselves aren't moved.
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
}
