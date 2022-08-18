use sqlite::Connection;

use crate::util::read_input;

pub fn add_project(db: &Connection) {
    let mut title = String::new();
    let mut description = String::new();
    let mut assignee = String::new();

    println!("Project title:");
    read_input(&mut title).expect("Failed to read project name");

    println!("Project description:");
    read_input(&mut description).expect("Failed to read project description");

    println!("Assign to team member:");
    read_input(&mut assignee).expect("Failed to read project description");
}
