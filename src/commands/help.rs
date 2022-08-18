struct Command {
    name: String,
    description: String,
    keys: Vec<String>,
}

impl Command {
    fn new(name: &str, description: &str, keys: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            keys,
        }
    }
}

pub fn print_help() {
    let command_log: Vec<Command> = vec![
        Command::new(
            "Help",
            "Get help",
            vec!["h".to_string(), "help".to_string()],
        ),
        Command::new(
            "Quit",
            "Quit the app",
            vec!["q".to_string(), "quit".to_string()],
        ),
        Command::new(
            "Add Project",
            "Add project to account",
            vec!["a".to_string()],
        ),
    ];

    for c in command_log.iter() {
        println!("{} - {} - {}", c.name, c.description, c.keys.join(", "));
    }
}
