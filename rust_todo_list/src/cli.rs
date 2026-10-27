use crate::dates::parse_due_date;
use crate::{priority::Priority, todo_filter::TodoFilter};
use chrono::NaiveDate;

const USAGE: &str = "Available commands:
  todo add <title> [-p <priority>] [-d <date>]
  todo list [-f <filter>]
  todo done <id>
  todo remove <id>
  todo priority <high|medium|low|none> <id>
  todo due <dd-mm-yyyy|dd-mm|today|tmr|none> <id>
  todo check

Filters: due, overdue, pending, done, skipped, high, medium, low";

#[derive(Debug)]
pub enum Command {
    Todo(TodoCommand),
    // Shop(ShopCommand),
}

#[derive(Debug)]
pub enum TodoCommand {
    Add {
        title: String,
        priority: Option<Priority>,
        due_date: Option<NaiveDate>,
    },
    List {
        filter: Option<TodoFilter>,
    },
    Done {
        id: usize,
    },
    Remove {
        id: usize,
    },
    Priority {
        id: usize,
        priority: Option<Priority>,
    },
    DueDate {
        id: usize,
        due_date: Option<NaiveDate>,
    },
    // Health report for the database. Takes no arguments or flags.
    Check,
}

#[derive(Debug)]
enum Flag {
    Priority(Option<Priority>), // -p / --priority <value>
    DueDate(Option<NaiveDate>), // -d / --by | --deadline | --due <value>
    Filter(TodoFilter),
    All, // --all (no value)
}

// pub enum ShopCommand {
//     Add { title: String },
//     List,
//     MarkBought { id: u32 },
//     MarkUnbought { id: u32 },
// }

pub fn parse(args: &[String]) -> Result<Command, String> {
    let (group, rest) = args
        .split_first()
        .ok_or_else(|| format!("No command provided.\n\n{USAGE}"))?;

    let (results, flags) = parse_flags(rest)?;

    match group.as_str() {
        "todo" => parse_todo(&results, &flags).map(Command::Todo),
        unknown => Err(format!("Unknown command: {unknown}\n\n{USAGE}")),
    }
}

// Turns "high" / "h" / "none" etc. into an Option<Priority>.
// "none" → Ok(None), valid priority → Ok(Some(..)), anything else → Err.
fn parse_priority_value(value: &str) -> Result<Option<Priority>, String> {
    match value.to_lowercase().as_str() {
        "none" => Ok(None),
        // parse gives Result<Priority, String>.
        // .map(Some) wraps the Ok value: Ok(High) → Ok(Some(High)).
        // Errors pass through untouched.
        _ => value.parse::<Priority>().map(Some),
    }
}

// Splits args into plain words (positional) and recognised flags.
// ["Go", "gym", "-p", "high"] → (["Go", "gym"], [Flag::Priority(Some(High))])
fn parse_flags(args: &[String]) -> Result<(Vec<String>, Vec<Flag>), String> {
    // Both need `mut` because we push into them in the loop.
    let mut positional = Vec::new();
    let mut flags = Vec::new();

    // An iterator walks the args one at a time.
    // It must be `mut` because calling .next() moves it forward.
    let mut iter = args.iter();

    // Keep taking the next argument until there are none left.
    // `arg` is a &String (a borrowed view of one argument).
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            // A flag that takes a value: the value is the NEXT argument.
            "-p" | "--priority" => {
                // Calling iter.next() again here takes the following argument.
                // That argument is consumed, so the loop won't see it again.
                // If there isn't one (user typed `-p` at the end), it's an error.
                let value = iter
                    .next()
                    .ok_or_else(|| format!("{arg} needs a value: high, medium, low or none"))?;

                flags.push(Flag::Priority(parse_priority_value(value)?));
            }

            "-d" | "--due" | "--by" | "--deadline" => {
                // Calling iter.next() again here takes the following argument.
                // That argument is consumed, so the loop won't see it again.
                // If there isn't one (user typed `-d` at the end), it's an error.
                let value = iter.next().ok_or_else(|| {
                    format!("{arg} needs a date: dd-mm-yyyy, dd-mm, today, tmr or none")
                })?;

                flags.push(Flag::DueDate(parse_due_date(value)?));
            }

            "-f" | "--filter" => {
                // Calling iter.next() again here takes the following argument.
                // That argument is consumed, so the loop won't see it again.
                // If there isn't one (user typed `-d` at the end), it's an error.
                let value = iter.next().ok_or_else(|| {
                    format!(
                        "{arg} needs a filter: due, overdue, pending, done, skipped, high, medium, low"
                    )
                })?;

                // TodoFilter implements FromStr (see todo_filter.rs), so .parse() works.
                flags.push(Flag::Filter(value.parse::<TodoFilter>()?));
            }

            // A flag with no value: just record that it was given.
            "--all" => flags.push(Flag::All),

            // A match "guard": the arm is used only if the `if` is true.
            // Starts with '-' but isn't a flag we know → reject it instead
            // of silently putting it in the title.
            unknown if unknown.starts_with('-') => {
                return Err(format!("Unknown flag: {unknown}"));
            }

            // Anything else is an ordinary word.
            _ => positional.push(arg.clone()),
        }
    }

    // Hand both lists back as a tuple.
    Ok((positional, flags))
}

fn parse_todo(args: &[String], flags: &[Flag]) -> Result<TodoCommand, String> {
    let (action, rest) = args
        .split_first()
        .ok_or_else(|| format!("Expected a todo subcommand\n\n{USAGE}"))?;

    // Each arm only picks the right function, so this match stays short.
    match action.as_str() {
        "add" if !rest.is_empty() => parse_add(rest, flags),
        "add" => Err("Usage: todo add <title>".to_string()),

        "list" if rest.is_empty() => parse_list(flags),
        "list" => Err("Usage: todo list".to_string()),

        "done" => parse_number(rest).map(|id| TodoCommand::Done { id }),
        "remove" => parse_number(rest).map(|id| TodoCommand::Remove { id }),
        "priority" => {
            let (priority, id) = rest
                .split_first()
                .ok_or_else(|| "Usage: todo priority <high|medium|low|none> <id>".to_string())?;
            let p = parse_priority_value(priority)?;

            parse_number(id).map(|id| TodoCommand::Priority { id, priority: p })
        }

        "due" | "by" | "deadline" | "due-date" => {
            let (due_date, id) = rest.split_first().ok_or_else(|| {
                "Usage: todo due <dd-mm-yyyy|dd-mm|today|tmr|none> <id>".to_string()
            })?;
            let d: Option<NaiveDate> = parse_due_date(due_date)?;

            parse_number(id).map(|id| TodoCommand::DueDate { id, due_date: d })
        }

        "check" if rest.is_empty() && flags.is_empty() => Ok(TodoCommand::Check),
        "check" => Err("Usage: todo check".to_string()),

        unknown => Err(format!("Unknown todo subcommand: {unknown}")),
    }
}

// `todo add <title> [-p <priority>] [-d <date>]`
fn parse_add(rest: &[String], flags: &[Flag]) -> Result<TodoCommand, String> {
    let mut priority = None; // stays None if no -p was given
    let mut due_date = None;

    for flag in flags {
        // flag is a &Flag (flags is borrowed)
        match flag {
            Flag::Priority(p) => priority = *p,
            Flag::DueDate(d) => due_date = *d,
            other => return Err(format!("{other:?} isn't valid for add")),
        }
    }

    Ok(TodoCommand::Add {
        title: rest.join(" "),
        priority,
        due_date,
    })
}

// `todo list [-f <filter>]`
fn parse_list(flags: &[Flag]) -> Result<TodoCommand, String> {
    let mut filter = None;

    for flag in flags {
        // flag is a &Flag (flags is borrowed)
        match flag {
            Flag::Filter(f) => filter = Some(*f),
            other => return Err(format!("{other:?} isn't valid for list")),
        }
    }
    Ok(TodoCommand::List { filter })
}

fn parse_number(args: &[String]) -> Result<usize, String> {
    match args {
        [value] => value
            .parse::<usize>()
            .map_err(|_| format!("Expected a numeric ID, got '{value}'")),
        [] => Err("Missing ID".to_string()),
        _ => Err("Expected exactly one ID".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, TodoCommand, parse};

    // Turns string literals into the Vec<String> that parse() expects.
    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn parses_check() {
        let command = parse(&args(&["todo", "check"])).unwrap();
        assert!(matches!(command, Command::Todo(TodoCommand::Check)));
    }

    #[test]
    fn check_rejects_extra_arguments_and_flags() {
        assert!(parse(&args(&["todo", "check", "now"])).is_err());
        assert!(parse(&args(&["todo", "check", "-f", "done"])).is_err());
    }

    #[test]
    fn empty_args_show_available_commands() {
        let error = parse(&[])
            .err()
            .expect("empty arguments should be rejected");

        assert!(error.contains("todo add <title>"));
        assert!(error.contains("todo list"));
        assert!(error.contains("todo done <id>"));
        assert!(error.contains("todo remove <id>"));
    }
}
