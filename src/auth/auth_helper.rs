use crate::util::{clean_for_sql, open_db, read_input, User};
use crypto_hash::{hex_digest, Algorithm};
use regex::Regex;
use sqlite::State;
use std::io::{self, Write};

pub fn handle_auth(flag: &mut bool) -> User {
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        println!("Login or Signup? (l/s)");
        match read_input(&mut input) {
            Err(e) => panic!("Error: {e}"),
            _ => (),
        }

        if input == "s" || input == "S" {
            let mut s = false;
            let user = handle_signup(&mut s).unwrap();
            // if sign up was successful set the flag for the rest of the app
            if s {
                *flag = true;
                println!("Welcome, {}.", user.name);
                break user;
            }
        } else if input == "l" || input == "L" {
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

fn handle_signup(flag: &mut bool) -> Result<User, ()> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    let (email, password) = prompt_email_pw();

    let valid = email_regex.is_match(&email);
    if !valid {
        println!("Invalid email");
        *flag = false;
        return Err(());
    }

    let mut name = String::new();
    print!("Name: ");
    io::stdout().flush().unwrap();
    read_input(&mut name).expect("Error reading input.");

    let mut role = String::new();
    print!("Role: ");
    io::stdout().flush().unwrap();
    read_input(&mut role).expect("Error reading input.");

    let exists = check_email_exists(&email);

    if exists {
        println!("Email {} already in use!", &email);
        *flag = false;
        return Err(());
    }

    let conn = open_db().expect("Error opening db");

    let insert_statement = clean_for_sql(format!(
        r#"insert into users (email, password, name, role) values ("{}", "{}", "{}", "{}")"#,
        email, password, name, role
    ));

    conn.execute(insert_statement)
        .expect("Error creating account");

    *flag = true;
    Ok(User::new(email, name, role))
}

fn prompt_email_pw() -> (String, String) {
    let mut email = String::new();
    let mut password = String::new();

    print!("Email: ");
    io::stdout().flush().unwrap();
    read_input(&mut email).expect("Error reading input.");

    print!("Password: ");
    io::stdout().flush().unwrap();
    read_input(&mut password).expect("Error reading input.");

    let hashed_pw = hex_digest(Algorithm::SHA256, &password.as_bytes());

    (email, hashed_pw)
}

fn handle_login() {
    let conn = open_db().expect("Error opening db");
    let (email, password) = prompt_email_pw();

    let check_statement =
        clean_for_sql(format!(r#"select * from users where email = "{}";"#, email));

    let mut statement = conn.prepare(check_statement).unwrap();

    while let State::Row = statement.next().unwrap() {
        if statement.read::<String>(1).unwrap() == email.to_owned()
            && statement.read::<String>(2).unwrap() == password
        {
            println!("LOGIN");
        }
    }
}

fn check_email_exists(e: &String) -> bool {
    let conn = open_db().expect("Error opening db");

    let check_statement = clean_for_sql(format!(r#"select * from users where email = "{}";"#, e));

    let mut statement = conn.prepare(check_statement).unwrap();

    while let State::Row = statement.next().unwrap() {
        if statement.read::<String>(1).unwrap() == e.to_owned() {
            return true;
        }
    }
    false
}
