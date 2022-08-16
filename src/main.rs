use util::{open_db, print_prompt, User};

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
            print_prompt();
            match util::read_input(&mut input) {
                Ok(i) => commands::command::handle_command(&mut i.to_string()),
                Err(e) => println!("Error processing input: {e}"),
            }
        } else {
            user = auth::auth_helper::handle_auth(&mut auth_flag, &conn);
        }

        input = String::new();
    }
}
