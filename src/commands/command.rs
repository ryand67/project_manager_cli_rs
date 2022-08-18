use crate::commands::*;
use crate::util::User;
use sqlite::Connection;

pub fn handle_command(command: &mut String, user: &User, conn: &Connection) {
    // mapping of commands
    match command.to_lowercase().as_str() {
        "exit" | "q" => {
            std::process::exit(0);
        }
        "h" | "help" => {
            print_help();
        }
        "a" => {
            add_project(conn);
        }
        _ => println!("Unrecognized command"),
    }
}
