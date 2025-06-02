use std::{
    env,
    io::{Write, stdin, stdout},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        println!(">");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline, which trim removes
        let mut parts = input.trim().split_whitespace();
        println!("Parts: {:?}", parts);
        let command = parts.next().unwrap();
        println!("Command: {}", command);

        let args = parts;
        println!("Args: {:?}", args);

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let child = Command::new(command).args(args).spawn();

                // gracefully handle malformed user input
                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
