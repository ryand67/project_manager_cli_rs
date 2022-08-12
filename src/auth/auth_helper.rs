use crate::util::{open_db, read_input};
use sqlite::State;

pub fn handle_auth() -> bool {
    let mut input = String::new();
    loop {
        println!("Login or Signup? (l/s)");
        match read_input(&mut input) {
            Err(e) => panic!("Error: {e}"),
            _ => (),
        }

        if input == "q" {
            std::process::exit(0);
        }

        if input == "s" {
            handle_signup();
        } else if input == "l" {
            handle_login();
        }
    }
}

fn handle_signup() {
    let conn = open_db().unwrap();
    let mut re = conn.prepare("select * from test").unwrap();

    while let State::Row = re.next().unwrap() {
        println!("{:?}", re.read::<String>(0).unwrap());
    }
}

fn handle_login() {
    ()
}
