mod auth;
mod commands;
mod util;

use auth::*;
use commands::*;
use util::*;

fn main() {
    let mut authFlag = false;

    loop {
        let mut input = String::new();
        print!("> ");
        if authFlag {
            match util::read_input(&mut input) {
                Ok(i) => commands::command::handle_command(&mut i.to_string()),
                Err(e) => println!("Error processing input: {e}"),
            }
        } else {
            authFlag = auth::auth_helper::handle_auth();
        }
    }
}
