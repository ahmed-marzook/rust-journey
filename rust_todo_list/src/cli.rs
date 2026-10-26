use crate::priority::Priority;
use chrono::{Datelike, Days, Local, NaiveDate};

const USAGE: &str =
    "Available commands:\n  todo add <title>\n  todo list\n  todo done <id>\n  todo remove <id>";

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
    List,
    Done {
        id: u32,
    },
    Remove {
        id: u32,
    },
    Priority {
        id: u32,
        priority: Option<Priority>,
    },
    DueDate {
        id: u32,
        due_date: Option<NaiveDate>,
    },
}

#[derive(Debug)]
enum Flag {
    Priority(Option<Priority>), // -p / --priority <value>
    DueDate(Option<NaiveDate>), // -d / --by | --deadline | --due <value>
    All,                        // --all (no value)
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

fn parse_todo(args: &[String], flags: &Vec<Flag>) -> Result<TodoCommand, String> {
    let (action, rest) = args
        .split_first()
        .ok_or_else(|| "Expected a todo subcommand".to_string())?;

    match action.as_str() {
        "add" if !rest.is_empty() => {
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

        "add" => Err("Usage: todo add <title>".to_string()),

        "list" if rest.is_empty() => Ok(TodoCommand::List),
        "list" => Err("Usage: todo list".to_string()),

        "done" => parse_id(rest).map(|id| TodoCommand::Done { id }),
        "remove" => parse_id(rest).map(|id| TodoCommand::Remove { id }),
        "priority" => {
            let (priority, id) = rest
                .split_first()
                .ok_or_else(|| "Usage: todo priority <high|medium|low|none> <id>".to_string())?;
            let p = match priority.to_lowercase().as_str() {
                "none" => None,
                _ => Some(priority.parse::<Priority>()?),
            };

            parse_id(id).map(|id| TodoCommand::Priority { id, priority: p })
        }

        "due" | "by" | "deadline" | "due-date" => {
            let (due_date, id) = rest.split_first().ok_or_else(|| {
                "Usage: todo due-date <dd-mm-yyyy|dd-mm|tmr|today|none> <id>".to_string()
            })?;
            let d: Option<NaiveDate> = parse_due_date(due_date)?;

            parse_id(id).map(|id| TodoCommand::DueDate { id, due_date: d })
        }

        unknown => Err(format!("Unknown todo subcommand: {unknown}")),
    }
}

fn parse_id(args: &[String]) -> Result<u32, String> {
    match args {
        [value] => value
            .parse::<u32>()
            .map_err(|_| format!("Expected a numeric ID, got '{value}'")),
        [] => Err("Missing ID".to_string()),
        _ => Err("Expected exactly one ID".to_string()),
    }
}

fn parse_due_date(value: &str) -> Result<Option<NaiveDate>, String> {
    let today = Local::now().date_naive();

    let date = match value.to_lowercase().as_str() {
        "none" => return Ok(None),
        "today" => today,
        "tmr" | "tomorrow" => today
            .checked_add_days(Days::new(1))
            .ok_or_else(|| "Date out of range".to_string())?,
        _ => parse_date_text(value, today)?,
    };

    if date < today {
        return Err(format!(
            "Due date {} is in the past",
            date.format("%d-%m-%Y")
        ));
    }

    Ok(Some(date))
}

// Parses "30-10-2026" (dd-mm-yyyy) or "30-10" (dd-mm, year filled in).
// Returns a plain NaiveDate; "none" etc. were handled by the caller.
fn parse_date_text(value: &str, today: NaiveDate) -> Result<NaiveDate, String> {
    // Attempt 1: a full date, dd-mm-yyyy.
    NaiveDate::parse_from_str(value, "%d-%m-%Y")
        // Attempt 2: only runs if attempt 1 failed.
        // `|_|` ignores attempt 1's error, because we're trying something else.
        .or_else(|_| {
            // Add the current year to the end: "30-10" → "30-10-2026",
            // then parse it with the same pattern as attempt 1.
            // The ? works inside a closure too: if this parse fails,
            // the closure returns that error immediately.
            let this_year =
                NaiveDate::parse_from_str(&format!("{value}-{}", today.year()), "%d-%m-%Y")?;

            // Already passed this year? The user almost certainly means next year,
            // e.g. typing "01-01" in October means next January.
            if this_year < today {
                NaiveDate::parse_from_str(&format!("{value}-{}", today.year() + 1), "%d-%m-%Y")
            } else {
                Ok(this_year)
            }
        })
        // If both attempts failed, replace chrono's error with a helpful message.
        .map_err(|_| format!("Invalid date '{value}'. Use dd-mm-yyyy, dd-mm, today, tmr or none"))
}

#[cfg(test)]
mod tests {
    use super::parse;

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
