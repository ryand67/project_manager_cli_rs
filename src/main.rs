use commands::handle_command;
use util::{open_db, print_prompt, read_input, User};

mod auth;
mod commands;
mod util;

fn main() {
    // Set "global" util vars
    let mut auth_flag = false;
    let mut user = User::default();
    let conn = open_db().expect("Error opening db");

    loop {
        let mut input = String::new();
        if auth_flag {
            print_prompt();
            match read_input(&mut input) {
                Ok(i) => handle_command(&mut i.to_string(), &user, &conn),
                Err(e) => println!("Error processing input: {e}"),
            }
        } else {
            user = auth::auth_helper::handle_auth(&mut auth_flag, &conn);
        }

        input = String::new();
    }
}
