pub fn handle_command(command: &mut String) {
    if command == "exit" || command.to_owned() == "q" {
        std::process::exit(0);
    }
}
