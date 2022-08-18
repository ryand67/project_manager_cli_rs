use std::io::{self, stdin, Write};

pub fn read_input(buffer: &mut String) -> Result<String, std::io::Error> {
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
    print!("> ");
    io::stdout().flush().unwrap();
}
