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
        create table if not exists users (userId integer unique primary key autoincrement,
        email varchar(255) unique not null,
        password varchar(255) not null,
        name varchar(255) not null,
        role varchar(255) not null);
        "#,
        )
        .expect("migration error");

    connection
        .execute(
            r#"
        create table if not exists projects
        (projectId integer unique primary key autoincrement,
        authorId integer not null,
        title varchar(255) not null,
        status varchar(255) check (status in ('backlog', 'in progress', 'done')) not null default 'backlog',
        foreign key(authorId) references users(userId)
        );
        "#,
        )
        .expect("migration error");

    Ok(connection)
}
