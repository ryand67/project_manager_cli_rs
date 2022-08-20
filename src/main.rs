use auth::greeting;
use commands::handle_command;
use crypto_hash::{hex_digest, Algorithm};
use std::env;
use util::{open_db, print_prompt, read_input, User};

mod auth;
mod commands;
mod util;

fn main() {
    // Set "global" util vars
    let mut auth_flag = false;
    let mut user = User::default();
    let conn = open_db().expect("Error opening db");

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 2 {
        let hashed_pw = hex_digest(Algorithm::SHA256, &args[1].as_bytes());

        match auth::handle_login(&conn, args[0].to_string(), hashed_pw) {
            Err(()) => println!("No account found."),
            Ok(u) => {
                user = u;
                auth_flag = true;
                greeting(&user);
            }
        }
    }

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
