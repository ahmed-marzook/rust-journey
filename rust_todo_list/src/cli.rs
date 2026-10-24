pub enum Command {
    Todo(TodoCommand),
    Shop(ShopCommand),
}

pub enum TodoCommand {
    Add {
        title: String,
    },
    List,
    Done {
        id: u32,
    },
    Remove {
        id: u32,
    },
}

pub enum ShopCommand {
    Add {
        title: String,
    },
    List,
    MarkBought {
        id: u32,
    },
    MarkUnbought {
        id: u32,
    },
}

pub fn parse(args: &[String]) -> Result<Command, String> {
    let (group, rest) = args
        .split_first()
        .ok_or_else(|| "Expected a command: todo or shop".to_string())?;

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
        "add" if !rest.is_empty() =>
            Ok(TodoCommand::Add {
                title: rest.join(" "),
            }),
        "add" => Err("Usage: todo add <title>".to_string()),

        "list" if rest.is_empty() => Ok(TodoCommand::List),

        "done" => parse_id(rest).map(|id| TodoCommand::Done { id }),
        "remove" => parse_id(rest).map(|id| TodoCommand::Remove { id }),

        unknown => Err(format!("Unknown todo subcommand: {unknown}")),
    }
}

fn parse_id(args: &[String]) -> Result<u32, String> {
    match args {
        [value] =>
            value.parse::<u32>().map_err(|_| format!("Expected a numeric ID, got '{value}'")),
        [] => Err("Missing ID".to_string()),
        _ => Err("Expected exactly one ID".to_string()),
    }
}
