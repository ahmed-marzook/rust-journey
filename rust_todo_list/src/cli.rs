use crate::priority::Priority;

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
}

#[derive(Debug)]
enum Flag {
    Priority(Option<Priority>), // -p / --priority <value>
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

    print!("flasg {:?}", &flags);

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

            for flag in flags {
                // flag is a &Flag (flags is borrowed)
                match flag {
                    Flag::Priority(p) => priority = *p,
                    other => return Err(format!("{other:?} isn't valid for add")),
                }
            }

            Ok(TodoCommand::Add {
                title: rest.join(" "),
                priority,
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
