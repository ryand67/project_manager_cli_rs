use crate::util::{clean_for_sql, open_db, read_input, User};
use crypto_hash::{hex_digest, Algorithm};
use regex::Regex;
use sqlite::{Connection, State};
use std::io::{self, Write};

pub fn handle_auth(flag: &mut bool, conn: &Connection) -> User {
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
            match handle_signup(conn) {
                Ok(u) => {
                    *flag = true;
                    break u;
                }
                Err(()) => continue,
            }
        } else if input == "l" || input == "L" {
            match handle_login(conn) {
                Ok(u) => {
                    *flag = true;
                    break u;
                }
                Err(()) => {
                    println!("Wrong email/password.");
                }
            }
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

fn handle_signup(conn: &Connection) -> Result<User, ()> {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();

    let (email, password) = prompt_email_pw();

    let valid = email_regex.is_match(&email);
    if !valid {
        println!("Invalid email");
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
        return Err(());
    }

    let insert_statement = clean_for_sql(format!(
        r#"insert into users (email, password, name, role) values ("{}", "{}", "{}", "{}")"#,
        email, password, name, role
    ));

    conn.execute(insert_statement)
        .expect("Error creating account");

    let id = get_user_id(&email, &conn);

    Ok(User::new(email, name, role, id))
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

fn handle_login(conn: &Connection) -> Result<User, ()> {
    let (email, password) = prompt_email_pw();

    let check_statement =
        clean_for_sql(format!(r#"select * from users where email = "{}";"#, email));

    let mut statement = conn.prepare(check_statement).unwrap();

    while let State::Row = statement.next().unwrap() {
        if statement.read::<String>(1).unwrap() == email.to_owned()
            && statement.read::<String>(2).unwrap() == password
        {
            return Ok(User::new(
                statement.read::<String>(1).unwrap(),
                statement.read::<String>(3).unwrap(),
                statement.read::<String>(4).unwrap(),
                statement.read::<i64>(0).unwrap(),
            ));
        }
    }

    Err(())
}

fn get_user_id(e: &String, conn: &Connection) -> i64 {
    let check_statement = clean_for_sql(format!(
        r#"select userId from users where email = "{}";"#,
        e
    ));

    let mut statement = conn.prepare(check_statement).unwrap();

    let mut result: i64 = 0;
    while let State::Row = statement.next().unwrap() {
        result = statement.read::<i64>(0).unwrap();
    }

    if result > 0 {
        result
    } else {
        -1
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
