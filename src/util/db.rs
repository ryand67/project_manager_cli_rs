use dotenv;
use sqlite::Connection;
use std::fmt::Error;

pub fn open_db() -> Result<Connection, Error> {
    let loc: String;

    match dotenv::var("DB_LOCATION") {
        Ok(c) => loc = c,
        Err(_) => panic!("No DB_LOCATION env var present"),
    }

    let connection = sqlite::open(loc).unwrap();

    // this is cause i'm too lazy to set up migrations rn
    connection
        .execute(
            r#"
        create table if not exists users (userId integer primary key autoincrement,
        email varchar(255) not null,
        password varchar(255) not null
        );
        "#,
        )
        .expect("migration error");
    Ok(connection)
}
