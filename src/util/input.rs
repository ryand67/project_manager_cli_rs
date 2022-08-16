use std::io::{self, stdin, Write};

pub fn read_input(input: &mut String) -> Result<String, std::io::Error> {
    match stdin().read_line(input) {
        Ok(_) => input.truncate(input.len() - 1),
        Err(e) => panic!("Error: {e}"),
    }

    Ok(input.clone())
}

pub fn clean_for_sql(input: String) -> String {
    let s = input.replace("\"", "\"");
    String::from(s)
}

pub fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}
