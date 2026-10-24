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
