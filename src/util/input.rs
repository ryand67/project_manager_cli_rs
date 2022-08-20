use std::io::{self, stdin, Write};
use termion::color;

pub fn read_input(prompt: String, buffer: &mut String) -> Result<String, std::io::Error> {
    print!("{}{}", color::Fg(color::LightGreen), prompt);
    io::stdout().flush().unwrap();
    match stdin().read_line(buffer) {
        Ok(_) => {
            buffer.truncate(buffer.len() - 1);
        }
        Err(e) => panic!("Error: {e}"),
    }

    Ok(buffer.trim().to_string())
}

pub fn clean_for_sql(input: String) -> String {
    let s = input.replace("\"", "\"");
    String::from(s)
}

pub fn print_prompt() {
    print!("{}> ", color::Fg(color::White));
    io::stdout().flush().unwrap();
}
