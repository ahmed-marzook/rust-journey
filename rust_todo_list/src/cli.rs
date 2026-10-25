const USAGE: &str =
    "Available commands:\n  todo add <title>\n  todo list\n  todo done <id>\n  todo remove <id>";

pub enum Command {
    Todo(TodoCommand),
    Shop(ShopCommand),
}

pub enum TodoCommand {
    Add { title: String },
    List,
    Done { id: u32 },
    Remove { id: u32 },
}

pub enum ShopCommand {
    Add { title: String },
    List,
    MarkBought { id: u32 },
    MarkUnbought { id: u32 },
}

pub fn parse(args: &[String]) -> Result<Command, String> {
    let (group, rest) = args
        .split_first()
        .ok_or_else(|| format!("No command provided.\n\n{USAGE}"))?;

    match group.as_str() {
        "todo" => parse_todo(rest).map(Command::Todo),
        unknown => Err(format!("Unknown command: {unknown}")),
    }
}

fn parse_todo(args: &[String]) -> Result<TodoCommand, String> {
    let (action, rest) = args
        .split_first()
        .ok_or_else(|| "Expected a todo subcommand".to_string())?;

    match action.as_str() {
        "add" if !rest.is_empty() => Ok(TodoCommand::Add {
            title: rest.join(" "),
        }),
        "add" => Err("Usage: todo add <title>".to_string()),

        "list" if rest.is_empty() => Ok(TodoCommand::List),
        "list" => Err("Usage: todo list".to_string()),

        "done" => parse_id(rest).map(|id| TodoCommand::Done { id }),
        "remove" => parse_id(rest).map(|id| TodoCommand::Remove { id }),

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
