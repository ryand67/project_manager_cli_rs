use sqlite::Connection;

use crate::util::read_input;

pub fn add_project(db: &Connection) {
    let mut title = String::new();
    let mut description = String::new();
    let mut assignee = String::new();

    read_input("Project title:".to_string(), &mut title).expect("Failed to read project name");

    println!("Project description:");
    read_input("Project description: ".to_string(), &mut description)
        .expect("Failed to read project description");

    println!("Assign to team member:");
    read_input("Assign to team member:".to_string(), &mut assignee)
        .expect("Failed to read project description");
}
