use std::io::{self, Write};

use util::{open_db, User};

mod auth;
mod commands;
mod util;

fn main() {
    let mut auth_flag = false;
    let mut user: User;
    let conn = open_db().expect("Error opening db");

    loop {
        let mut input = String::new();
        if auth_flag {
            print!("> ");
            io::stdout().flush().unwrap();
            match util::read_input(&mut input) {
                Ok(i) => commands::command::handle_command(&mut i.to_string()),
                Err(e) => println!("Error processing input: {e}"),
            }
        } else {
            user = auth::auth_helper::handle_auth(&mut auth_flag, &conn);
            println!("Welcome to the app, {:?}.", user.name);
        }

        input = String::new();
    }
}
