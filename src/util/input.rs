use std::io::stdin;

pub fn read_input(input: &mut String) -> Result<String, std::io::Error> {
    match stdin().read_line(input) {
        Ok(_) => input.truncate(input.len() - 1),
        Err(e) => panic!("Error: {e}"),
    }

    Ok(input.clone())
}
