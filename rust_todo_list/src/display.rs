use crate::db::Health;
use crate::status::Status;
use crate::task::Task;
use crate::todo_filter::TodoFilter;
use chrono::NaiveDate;

// Prints the numbered tasks as a table.
// `tasks` comes from TodoList::visible_tasks, already sorted, numbered and filtered.
pub fn print_tasks(tasks: &[(usize, &Task)], today: NaiveDate) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    // Header, using the SAME widths as the rows below so the columns line up.
    //   {:>3}  right-aligned, 3 wide  (numbers look best right-aligned)
    //   {:<4}  left-aligned, 4 wide
    println!(
        "{:>3}  {:<4}  {:<8}  {:<10}  {}",
        "#", "Done", "Priority", "Due", "Title"
    );
    println!("{}", "-".repeat(50)); // a divider line: "-" repeated 50 times

    for (index, task) in tasks {
        println!("{}", format_row(*index, task, today));
    }
}

// Builds one table row as a String, so it can be tested without printing.
fn format_row(index: usize, task: &Task, today: NaiveDate) -> String {
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

    format!(
        "{:>3}  {:<4}  {:<8}  {:<10}  {}{}",
        index + 1,
        done,
        priority,
        due,
        task.title,
        overdue
    )
}

// Prints the `todo check` report, one labelled line per check.
pub fn print_health(health: &Health, path: &str) {
    // {:<16} pads each label to the same width so the values line up.
    println!("{:<16}{}", "Database:", path);
    println!(
        "{:<16}{} (latest {})",
        "Schema version:", health.schema_version, health.latest_version
    );
    println!("{:<16}{}", "Integrity:", health.integrity);
    println!("{:<16}{}", "Tasks:", health.task_count);
}
