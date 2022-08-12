use crate::util::{open_db, read_input};
use crypto_hash::{hex_digest, Algorithm};
use regex::Regex;
use sqlite::{Connection, State};
use std::io::{self, Write};

pub fn handle_auth(flag: &mut bool) -> bool {
    let mut input = String::new();
    loop {
        println!("Login or Signup? (l/s)");
        match read_input(&mut input) {
            Err(e) => panic!("Error: {e}"),
            _ => (),
        }

        if input == "s" {
            let mut s = false;
            handle_signup(&mut s);
            if !s {
                continue;
            } else {
                *flag = true;
            }
        } else if input == "l" {
            handle_login();
        } else if input == "q" {
            std::process::exit(0);
        } else {
            println!("Unrecognized command.");
            io::stdout().flush().unwrap();
        }

        // If not re-init'd, the input will just gather commands
        input = String::new();
    }
}

fn handle_signup(flag: &mut bool) {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    let mut email = String::new();
    let mut password = String::new();

    print!("Email: ");
    io::stdout().flush().unwrap();
    read_input(&mut email).expect("Error reading input.");
    let valid = email_regex.is_match(&email);
    if !valid {
        println!("Invalid email");
        *flag = false;
        return;
    }

    print!("Password: ");
    io::stdout().flush().unwrap();
    read_input(&mut password).expect("Error reading input.");

    let hashed_pw = hex_digest(Algorithm::SHA256, &password.as_bytes());

    let conn = open_db().expect("Error opening db");
    // let mut re = conn.prepare("select * from test").unwrap();

    // while let State::Row = re.next().unwrap() {
    //     println!("{:?}", re.read::<String>(0).unwrap());
    // }

    let s = format!(r#"select * from users where email = "{}";"#, email).replace("\"", "\"");

    conn.iterate(s, |pairs| {
        for &(col, val) in pairs.iter() {
            println!("{} = {}", col, val.unwrap());
        }
        true
    })
    .unwrap();
    *flag = false;
}

fn handle_login() {
    todo!();
}
