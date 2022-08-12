use std::io::{self, Write};

mod auth;
mod commands;
mod util;

fn main() {
    let mut auth_flag = false;

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        if auth_flag {
            match util::read_input(&mut input) {
                Ok(i) => commands::command::handle_command(&mut i.to_string()),
                Err(e) => println!("Error processing input: {e}"),
            }
        } else {
            auth::auth_helper::handle_auth(&mut auth_flag);
        }

        input = String::new();
    }
}
