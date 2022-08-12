use std::io::{self, Write};

use util::User;

mod auth;
mod commands;
mod util;

fn main() {
    let mut auth_flag = false;
    let mut user: User;

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
            user = auth::auth_helper::handle_auth(&mut auth_flag);
        }

        input = String::new();
    }
}
